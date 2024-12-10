// use std::collections::HashMap;

// /// A node in the Trie structure
// struct TrieNode {
//     /// Indicates if this node represents the end of a word
//     is_word: bool,
//     /// Maps characters to child nodes
//     children: HashMap<char, TrieNode>,
// }

// impl TrieNode {
//     fn new() -> Self {
//         TrieNode {
//             is_word: false,
//             children: HashMap::new(),
//         }
//     }
// }

// /// Trie data structure for efficient string operations
// pub struct Trie {
//     root: TrieNode,
// }

// impl Trie {
//     /// Creates a new empty Trie
//     pub fn new() -> Self {
//         Trie {
//             root: TrieNode::new(),
//         }
//     }

//     /// Inserts a word into the Trie
//     pub fn insert(&mut self, word: &str) {
//         let mut current_node = &mut self.root;
        
//         for c in word.chars() {
//             current_node = current_node.children
//                 .entry(c)
//                 .or_insert(TrieNode::new());
//         }
        
//         current_node.is_word = true;
//     }

//     /// Returns true if the word exists in the Trie
//     pub fn search(&self, word: &str) -> bool {
//         let mut current_node = &self.root;
        
//         for c in word.chars() {
//             match current_node.children.get(&c) {
//                 Some(node) => current_node = node,
//                 None => return false,
//             }
//         }
        
//         current_node.is_word
//     }

//     /// Returns true if there is any word in the Trie that starts with the given prefix
//     pub fn starts_with(&self, prefix: &str) -> bool {
//         let mut current_node = &self.root;
        
//         for c in prefix.chars() {
//             match current_node.children.get(&c) {
//                 Some(node) => current_node = node,
//                 None => return false,
//             }
//         }
        
//         true
//     }

//     /// Returns all words in the Trie with the given prefix
//     pub fn find_words_with_prefix(&self, prefix: &str) -> Vec<String> {
//         let mut result = Vec::new();
        
//         // First, navigate to the prefix node
//         let mut current_node = &self.root;
//         for c in prefix.chars() {
//             match current_node.children.get(&c) {
//                 Some(node) => current_node = node,
//                 None => return result,
//             }
//         }
        
//         // Then perform DFS to collect all words
//         let mut prefix = String::from(prefix);
//         self.collect_words(current_node, &mut prefix, &mut result);
        
//         result
//     }

//     /// Helper function to collect words using DFS
//     fn collect_words(&self, node: &TrieNode, prefix: &mut String, result: &mut Vec<String>) {
//         if node.is_word {
//             result.push(prefix.clone());
//         }
        
//         for (&c, child) in &node.children {
//             prefix.push(c);
//             self.collect_words(child, prefix, result);
//             prefix.pop();
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_trie_operations() {
//         let mut trie = Trie::new();
        
//         // Test insert and search
//         trie.insert("hello");
//         trie.insert("world");
//         trie.insert("hell");
        
//         assert!(trie.search("hello"));
//         assert!(trie.search("hell"));
//         assert!(!trie.search("hel"));
        
//         // Test prefix search
//         assert!(trie.starts_with("hel"));
//         assert!(!trie.starts_with("hit"));
        
//         // Test finding words with prefix
//         let words = trie.find_words_with_prefix("hel");
//         assert_eq!(words, vec!["hell", "hello"]);
//     }
// }



// Alternative implementation:
use std::collections::HashMap;

struct TrieNode {
    is_word: bool,
    children: HashMap<char, TrieNode>
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            is_word: false,
            children: HashMap::new()
        }
    }
}

struct Trie {
    root: TrieNode
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Trie {
            root: TrieNode::new()
        }
    }
    
    fn insert(&mut self, word: String) {
        let mut curr = &mut self.root;
        for c in word.chars() {
            curr = curr.children
                    .entry(c) // gets or insert with this key
                    .or_insert(TrieNode::new());
            // if !curr.children.contains_key(&c) {
            //     curr.children.insert(c, TrieNode::new());
            // }

            // // Get mutable reference to the child node
            // curr = curr.children.get_mut(&c).unwrap();
        }

        curr.is_word = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut current = &self.root;
        
        for c in word.chars() {
            match current.children.get(&c) {
                Some(node) => current = node,
                None => return false,
            }
        }
        
        current.is_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut current = &self.root;
        
        for c in prefix.chars() {
            match current.children.get(&c) {
                Some(node) => current = node,
                None => return false,
            }
        }
        
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_operations() {
        let mut trie = Trie::new();
        
        // Test insert and search
        trie.insert("hello".to_string());
        assert!(trie.search("hello".to_string()));
        assert!(!trie.search("hell".to_string()));
        assert!(!trie.search("helloa".to_string()));

        // Test starts_with
        assert!(trie.starts_with("hel".to_string()));
        assert!(!trie.starts_with("abc".to_string()));

        // Test multiple insertions
        trie.insert("hell".to_string());
        assert!(trie.search("hell".to_string()));
        assert!(trie.search("hello".to_string()));
    }
}