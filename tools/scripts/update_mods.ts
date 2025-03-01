import { dirname, fromFileUrl } from "path";

/////////////
// HELPERS //
/////////////

function lsDir(path: string): {
  files: string[];
  folders: string[];
} {
  return Deno.readDirSync(path).reduce(
    (acc, entry) => {
      if (
        entry.isFile && entry.name.endsWith(".rs") && entry.name !== "mod.rs"
      ) {
        acc.files.push(entry.name);
      }
      if (entry.isDirectory) acc.folders.push(entry.name);
      return acc;
    },
    { files: [], folders: [] } as {
      files: string[];
      folders: string[];
    },
  );
}

function readFile(path: string): string {
  const moduleBytes = Deno.readFileSync(path);
  const moduleString = new TextDecoder().decode(moduleBytes);
  return moduleString;
}

function writeFile(path: string, content: string): void {
  const file = Deno.openSync(path, {
    create: true,
    write: true,
    truncate: true,
  });
  file.writeSync(new TextEncoder().encode(content));
  file.close();
}

/////////////

function getAllPublicFunctionsFromTheModule(
  fileContent: string,
): string[] {
  const regex = /pub\sfn\s(\w+)\(/g;
  return (fileContent.match(regex) || []).map((x) =>
    x.replace("pub fn ", "").replace("(", "")
  );
}

function ensureModRsInModuleDir(
  parentPath: string,
  directory: string,
): boolean {
  const { files, folders } = lsDir(`${parentPath}/${directory}`);

  const modLines: string[] = [];
  const fileModulePreludes: string[] = [];

  for (let i = 0; i < folders.length; i++) {
    if (ensureModRsInModuleDir(`${parentPath}/${directory}`, folders[i])) {
      modLines.push(`pub mod ${folders[i]};`);
    }
  }

  if (files.length && modLines.length) {
    modLines.push("");
  }

  for (let i = 0; i < files.length; i++) {
    const moduleName = files[i].replace(".rs", "");

    modLines.push(`pub mod ${moduleName};`);

    const publicFunctions = getAllPublicFunctionsFromTheModule(
      readFile(`${parentPath}/${directory}/${files[i]}`),
    );

    if (publicFunctions.length === 1) {
      fileModulePreludes.push(
        `pub use super::${moduleName}::${publicFunctions[0]} as ${moduleName};`,
      );
    } else if (publicFunctions.length > 1) {
      fileModulePreludes.push(
        `pub use super::${moduleName}::*;`,
      );
    }
  }

  if (modLines.length) {
    modLines.unshift("#![allow(unused_imports)]\n");
    modLines.unshift("// Maintained by ./tools/update_mods.sh\n");

    if (fileModulePreludes.length) {
      modLines.push("\npub mod prelude {");
      modLines.push(...fileModulePreludes);
      modLines.push("}");
    }

    modLines.push("");

    writeFile(`${parentPath}/${directory}/mod.rs`, modLines.join("\n"));
    return true;
  }

  return false;
}

////////
// GO //
////////

const projectRoot = dirname(dirname(dirname(fromFileUrl(import.meta.url))));
const codeSource = `${projectRoot}/src`;
const dirsToScan = ["exercises"];

for (let i = 0; i < dirsToScan.length; i++) {
  ensureModRsInModuleDir(codeSource, dirsToScan[i]);
}

console.log(`✅ mod.rs files updated successfully!`);
