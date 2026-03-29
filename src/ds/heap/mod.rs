use crate::ds::heap::heap::{Heap, HeapPub};

pub mod heap;

pub fn test_heap() {
    let mut heap: Heap<usize> = Heap::new();
    heap.insert(100);
    heap.insert(88);
    heap.insert(25);
    heap.insert(87);
    heap.insert(16);
    heap.insert(8);
    heap.insert(12);
    heap.insert(86);
    heap.insert(50);
    heap.insert(2);
    heap.insert(15);
    heap.insert(3);
    
    println!("El inicio de la Heap es: {:?}", heap.root_node());
    println!("El final de la Heap es: {:?}", heap.last_node());
    
    println!("Despues de eliminar el elemento root ({}), el inicio del heap es: {}", heap.delete().unwrap(), heap.root_node());
    println!("Y el final de la Heap es: {}", heap.last_node());
    
    let value = 250;
    heap.insert(value);
    println!("Despues de insertar el elemento root ({}), el inicio del heap es: {}", value, heap.root_node());
    println!("Y el final de la Heap es: {}", heap.last_node());
}