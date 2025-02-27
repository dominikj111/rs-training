#!/Users/dominikjelinek/cli/deno-run

import { path } from "/Users/dominikjelinek/cli/packages.ts";

import "/Users/dominikjelinek/cli/types.ts";

const { dirname, fromFileUrl } = path;

const projectRoot = dirname(dirname(fromFileUrl(import.meta.url)));
const codeSource = `${projectRoot}/src`;
const dirsToScan = ["exercises"];

function ensureModRsInModuleDir(
  parentPath: string,
  directory: string
): boolean {
  const { files, folders } = Deno.readDirSync(
    `${parentPath}/${directory}`
  ).reduce(
    (acc, entry) => {
      if (entry.isFile && entry.name.endsWith(".rs") && entry.name !== "mod.rs")
        acc.files.push(entry.name);
      if (entry.isDirectory) acc.folders.push(entry.name);
      return acc;
    },
    { files: [], folders: [] } as {
      files: string[];
      folders: string[];
    }
  );

  const modLines: string[] = [];

  for (let i = 0; i < folders.length; i++) {
    if (ensureModRsInModuleDir(`${parentPath}/${directory}`, folders[i])) {
      modLines.push(`pub mod ${folders[i]};`);
    }
  }

  if (files.length && modLines.length) {
    modLines.push("");
  }

  for (let i = 0; i < files.length; i++) {
    modLines.push(`pub mod ${files[i].replace(".rs", "")};`);
  }

  if (modLines.length) {
    modLines.unshift("// Maintained by ./tools/update_mods.ts\n");

    const file = Deno.openSync(`${parentPath}/${directory}/mod.rs`, {
      create: true,
      write: true,
      truncate: true,
    });

    file.writeSync(new TextEncoder().encode(modLines.join("\n") + "\n"));
    file.close();

    return true;
  }

  return false;
}

for (let i = 0; i < dirsToScan.length; i++) {
  ensureModRsInModuleDir(codeSource, dirsToScan[i]);
}

console.log(`✅ mod.rs files updated successfully!`);
