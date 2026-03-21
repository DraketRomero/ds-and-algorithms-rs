pub mod double_linked_list;
pub mod linked_list;
pub mod binary_search_tree;

use linked_list::LinkedList;

use crate::ds::node_base::{binary_search_tree::BinarySearchTree, double_linked_list::DoubleLinkedList};

pub fn test_ll() {
    let mut linked_list: LinkedList<&str> = LinkedList::new();

    println!(
        "El primer valor de la lista es: {:?} con una longitud de {}\nDespues de agregar algunos elementos:",
        linked_list.head, linked_list.length
    );
    linked_list.insert_at_index(0, "once");
    linked_list.insert_at_index(1, "upon");
    linked_list.insert_at_index(2, "a");
    linked_list.insert_at_index(3, "time");

    linked_list.print_elements();

    let element = "There ";
    let index = 0;
    println!(
        "\nQuiero agregar el elemento: '{}' en el indice: {}",
        &element, &index
    );
    linked_list.insert_at_index(index, &element);

    linked_list.print_elements();

    println!(
        "\nPor ultimo quiero eliminar el elemento en el indice: {}\nPor lo que quedan: ",
        1
    );

    linked_list.delete_at_index(2);

    println!("El ultimo elemento de la lista es: {:?}", linked_list.get_last_element());

    println!("Elementos antes de invertirlos:");
    linked_list.print_elements();
    
    linked_list.reverse_list();

    println!("Elementos despues  de invertirlos:");
    linked_list.print_elements();

    println!("El ultimo elemento de la lista es: {:?}", linked_list.get_last_element());
}

pub fn test_double_ll() {
    let mut double_ll: DoubleLinkedList<String> = DoubleLinkedList::new();

    println!(
        "El primer valor de la lista es: {:?} con una longitud de {}\nDespues de agregar algunos elementos:",
        double_ll.first_node, double_ll.length
    );
    double_ll.insert_at_end(String::from("Hola"));
    double_ll.insert_at_end(String::from("esta"));
    double_ll.insert_at_end(String::from("es"));
    double_ll.insert_at_end(String::from("una"));
    double_ll.insert_at_end(String::from("prueba"));

    println!("\nLos elementos leidos de inicio a fin:");
    double_ll.print_from_firs_to_last_node();

    println!("\nLos elementos leidos de forma inversa:");
    double_ll.print_from_last_to_first_node();
}

pub fn test_search_binary_tree() {
    let mut tree: BinarySearchTree<usize> = BinarySearchTree::new();

    tree.insert(50);
    tree.insert(75);
    tree.insert(25);
    tree.insert(10);
    tree.insert(33);
    tree.insert(56);
    tree.insert(89);
    tree.insert(4);
    tree.insert(11);
    tree.insert(30);
    tree.insert(40);
    tree.insert(52);
    tree.insert(61);
    tree.insert(82);
    tree.insert(95);


    let search_value = 50;
    match tree.search(&tree.root, search_value) {
        Some(value) => println!("El valor {:?} buscado en el arbol esta.", value.value),
        None => println!("El valor {} buscado en el arbol NO esta.", search_value)
    }

    println!("El root es: {:#?}", tree.root);

    println!("Despues de eliminar el elementos {}", search_value);

    tree.delete(search_value);
    tree.delete(4);
    tree.delete(10);
    
    println!("El root es: {:#?}", tree.root);

    println!("El valor de mas alto valor en el arbol es: {:?}", tree.find_greatest());
}