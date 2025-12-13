// This chapter is dedicated to the smart pointers: Box, Rc and RefCell.

use std::cell::RefCell;
use std::rc::Rc;

// Box
// ================================================================================================

// ----- 1 --------------------------------------
// Implement a recursive `BinaryTreeNode` which have:
// - fields:
//   - `value: i32`
//   - `left_child: Option<BinaryTreeNode>`
//   - `right_child: Option<BinaryTreeNode>`
// - methods:
//   - `new(value: i32)`, which creates a note with provided value and without any children
//   - `with_children(value: i32, left_child: BinaryTreeNode, right_child: BinaryTreeNode)` which
//     creates a note using the provided values
//   - `sum(&self)` which computes the sum of all values in the tree
//
// Use `Box` if needed

#[derive(Debug)]
pub struct BinaryTreeNode {
    value: i32,
    left_child: Option<Box<BinaryTreeNode>>,
    right_child: Option<Box<BinaryTreeNode>>,
}

impl BinaryTreeNode {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            left_child: None,
            right_child: None,
        }
    }

    pub fn with_children(
        value: i32,
        left_child: BinaryTreeNode,
        right_child: BinaryTreeNode,
    ) -> Self {
        Self {
            value,
            left_child: Some(Box::new(left_child)),
            right_child: Some(Box::new(right_child)),
        }
    }

    pub fn sum(&self) -> i32 {
        let mut result = self.value;

        if let Some(ref left) = self.left_child {
            result += left.sum();
        }
        if let Some(ref right) = self.right_child {
            result += right.sum();
        }

        result
    }
}
// Rc
// ================================================================================================

// ----- 2 --------------------------------------
// Implement a package dependency tree where multiple packages can depend on the same shared
// library.
//
// Implement the `Package` struct with `name: String` and `dependencies: Vec<Package>` fields.
// Implement methods:
// - `new(name: &str) -> Self` which creates a new package with provided name and without any
//   dependencies.
// - `with_dependencies(name: &str, dependencies: Vec<Package>) -> Self` which creates a new package
//   with provided name and dependencies.
// - `list_dependencies(package: Package) -> Vec<String>` which return a vector of all dependencies
//   of this package (including all recursive dependencies).
//
// Write a test which will reuse the created Packages in several other Packages as dependencies.
// Use `Rc` in the `Package` struct where needed to avoid deep clone.

// IMPLEMENT HERE:

#[derive(Debug)]
pub struct Package {
    name: String,
    dependencies: Vec<Rc<Package>>,
}

impl Package {
    pub fn new(name: &str) -> Rc<Self> {
        Rc::new(Self {
            name: name.to_string(),
            dependencies: Vec::new(),
        })
    }

    pub fn with_dependencies(name: &str, dependencies: Vec<Rc<Package>>) -> Rc<Self> {
        Rc::new(Self { name: name.to_string(), dependencies })
    }

    pub fn list_dependencies(package: Rc<Package>) -> Vec<String> {
        fn collect(pkg: &Rc<Package>, acc: &mut Vec<String>) {
            for dep in &pkg.dependencies {
                acc.push(dep.name.clone());
                collect(dep, acc);
            }
        }

        let mut result = Vec::new();
        collect(&package, &mut result);
        result
    }
}

#[test]
fn test_list_dependencies() {
    let shared_lib = Package::new("shared-lib");
    let utils = Package::with_dependencies("utils", vec![Rc::clone(&shared_lib)]);
    let core = Package::with_dependencies("core", vec![Rc::clone(&shared_lib)]);
    let app = Package::with_dependencies("app", vec![utils, core]);

    let deps = Package::list_dependencies(app);

    assert!(deps.contains(&"utils".to_string()));
    assert!(deps.contains(&"core".to_string()));
    assert_eq!(deps.iter().filter(|d| *d == "shared-lib").count(), 2);
}

// RefCell
// ================================================================================================

// ----- 3 --------------------------------------
// Create a simple `SharedCounter` where multiple owners can increment its value without mutable
// reference.
//
// Implement `new() -> Self` constructor, `increment(&self)` and `get(&self) -> i32` methods.
// Use `RefCell` where needed.

pub struct SharedCounter {
    value: RefCell<i32>,
}

impl SharedCounter {
    pub fn new() -> Self {
        Self { value: RefCell::new(0) }
    }

    pub fn increment(&self) {
        *self.value.borrow_mut() += 1;
    }

    pub fn get(&self) -> i32 {
        *self.value.borrow()
    }
}
