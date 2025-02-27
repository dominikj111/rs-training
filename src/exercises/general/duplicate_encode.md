/**
 * The goal of this exercise is to convert a string to a new string where each character in the new string
 * is "(" if that character appears only once in the original string, or ")" if that character appears more
 * than once in the original string. Ignore capitalization when determining if a character is a duplicate.
 */

 // some possible solution

  // let chars: Vec<char> = word.to_lowercase().chars().collect();
    // let result: String = chars
    // 	.iter()
    // 	.map(|&x| {
    // 		if
    // 			chars
    // 				.iter()
    // 				.filter(|&c| *c == x)
    // 				.count() == 1
    // 		{
    // 			'('
    // 		} else {
    // 			')'
    // 		}
    // 	})
    // 	.collect();
    // result