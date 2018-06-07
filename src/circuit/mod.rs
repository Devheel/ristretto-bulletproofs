//! scratch work on circuit data structures

use petgraph::graph::DiGraph;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Gate {
    /// Fan-in 0
    Input,
    /// Fan-in 0
    Constant,
    /// Fan-in 2
    Add,
    /// Fan-in 2
    Mul,
}

impl Default for Gate {
    fn default() -> Gate {
        Gate::Constant
    }
}

type Circuit = DiGraph<Gate, ()>;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn circuit_scratch() {
        let mut bootle16 = Circuit::new();

        let x1 = bootle16.add_node(Gate::Input);
        let y1 = bootle16.add_node(Gate::Input);
        let x2 = bootle16.add_node(Gate::Input);
        let y2 = bootle16.add_node(Gate::Input);
        let x3 = bootle16.add_node(Gate::Input);
        let y3 = bootle16.add_node(Gate::Input);

        let four = bootle16.add_node(Gate::Constant);

        let mul1 = bootle16.add_node(Gate::Mul);
        let mul2 = bootle16.add_node(Gate::Mul);
        let mul3 = bootle16.add_node(Gate::Mul);
        let mul4 = bootle16.add_node(Gate::Mul);
        let mul5 = bootle16.add_node(Gate::Mul);
        let mul6 = bootle16.add_node(Gate::Mul);
        let mul7 = bootle16.add_node(Gate::Mul);

        let add1 = bootle16.add_node(Gate::Add);

        bootle16.extend_with_edges(&[
            (x1, mul1),
            (y1, mul1),
            (x2, mul2),
            (y2, mul2),
            (x3, mul3),
            (y3, mul3),
            (mul1, mul4),
            (mul2, mul4),
            (mul3, mul5),
            (four, mul5),
            (mul4, mul6),
            (add1, mul6),
            (mul4, add1),
            (mul5, add1),
            (mul5, mul7),
            (add1, mul7),
        ]);

        println!("\n\nTest circuit:");
        println!("{:?}", bootle16);

        use petgraph::dot::{Dot, Config};

        println!("\n\nGraphviz:");
        println!("{:?}", Dot::with_config(&bootle16, &[Config::EdgeNoLabel]));

        panic!();
    }
}
