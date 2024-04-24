use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Port {
    pub value: i32,
}

pub trait Operator {
    fn compute (&mut self);
    fn get_input_port(&self, port_id: &usize) -> Option<&Rc<RefCell<Port>>>;
    fn get_output_port(&self, port_id: &usize) -> Option<&Rc<RefCell<Port>>>;
}

pub struct OperatorAdd {
    input_ports: HashMap<usize, Rc<RefCell<Port>>>,
    output_ports: HashMap<usize, Rc<RefCell<Port>>>,
}

impl OperatorAdd {
    pub fn new() -> Self {
        let mut input_ports = HashMap::new();
        let mut output_ports = HashMap::new();
        
        for i in 0..2 {
            input_ports.insert(i, Rc::new(RefCell::new(Port { value: 0 })));
        }
        
        for i in 0..1 {
            output_ports.insert(i, Rc::new(RefCell::new(Port { value: 0 })));
        }

        OperatorAdd {
            input_ports,
            output_ports,
        }
    }
}

impl Operator for OperatorAdd {
    fn compute(&mut self) {
        let total: i32 = self.input_ports.values()
            .map(|port| port.borrow().value)
            .sum();

        for port in self.output_ports.values_mut() {
            port.borrow_mut().value = total;
        }
    }

    fn get_input_port(&self, port_id: &usize) -> Option<&Rc<RefCell<Port>>> {
        self.input_ports.get(port_id)
    }

    fn get_output_port(&self, port_id: &usize) -> Option<&Rc<RefCell<Port>>> {
        self.output_ports.get(port_id)
    }
}

// struct OperatorMinus {
//     input_port: Vec<Rc<RefCell<i32>>>,
//     output_port: Rc<RefCell<i32>>,
// }

struct OperatorInput {
    output_ports: Vec<Rc<RefCell<Port>>>,
}

struct OperatorOutput {
    input_ports: Vec<Rc<RefCell<Port>>>,
}

impl OperatorInput {
    pub fn new(tracks: usize) -> Self {
        let output_ports = (0..tracks).map(|_| Rc::new(RefCell::new(Port { value: 0 }))).collect();
        OperatorInput { output_ports }
    }
}

impl OperatorOutput {
    pub fn new(tracks: usize) -> Self {
        let input_ports = (0..tracks).map(|_| Rc::new(RefCell::new(Port { value: 0 }))).collect();
        OperatorOutput { input_ports }
    }
}

impl Operator for OperatorInput {
    fn compute(&mut self) {
        
    }

    fn get_input_port(&self, _port_id: &usize) -> Option<&Rc<RefCell<Port>>> {
        None
    }

    fn get_output_port(&self, port_id: &usize) -> Option<&Rc<RefCell<Port>>> {
        self.output_ports.get(*port_id)
    }
}

impl Operator for OperatorOutput {
    fn compute(&mut self) {
        
    }

    fn get_input_port(&self, port_id: &usize) -> Option<&Rc<RefCell<Port>>> {
        self.input_ports.get(*port_id)
    }

    fn get_output_port(&self, _port_id: &usize) -> Option<&Rc<RefCell<Port>>> {
        None
    }
}

pub struct Connection {
    pub src_node_id: usize,
    pub src_port_id: usize,
    pub dst_node_id: usize,
    pub dst_port_id: usize,
}

pub struct OperatorRack {
    pub operators: HashMap<usize, Box<dyn Operator>>,
    pub connections: Vec<Connection>,
}

impl OperatorRack {
    pub fn new(track_number: usize) -> Self {
        let mut rack = OperatorRack {
            operators: HashMap::new(),
            connections: Vec::new(),
        };

        let input_node = Box::new(OperatorInput::new(track_number));
        let output_node = Box::new(OperatorOutput::new(track_number));

        // 假设我们为它们分配固定的ID，例如0和1
        rack.add_node(0, input_node);
        rack.add_node(1, output_node);

        rack
    }

    pub fn add_node(&mut self, node_id: usize, node: Box<dyn Operator>) {
        self.operators.insert(node_id, node);
    }

    pub fn connect(&mut self, src_node_id: usize, src_port_id: usize, dst_node_id: usize, dst_port_id: usize) {
        self.connections.push(Connection {
            src_node_id,
            src_port_id,
            dst_node_id,
            dst_port_id,
        });
    }

    pub fn compute(&mut self) {
        // 根据当前的连接和操作节点，进行拓扑排序
        let sorted_nodes = Self::topological_sort(&self.operators, &self.connections);
        println!("Sorted Nodes for Computation: {:?}", sorted_nodes);

        // 级联更新输入并计算输出
        for node_id in &sorted_nodes {
            println!("Updating inputs for node {}", node_id);
            // 更新当前节点的输入
            self.update_inputs(*node_id);

            // 立即计算当前节点的输出
            if let Some(node) = self.operators.get_mut(node_id) {
                println!("Computing for node {}", node_id);
                node.compute();
                if let Some(output) = node.get_output_port(&0) {
                    println!("After compute: Node {} Output port 0 value {}", node_id, output.borrow().value);
                }
            }
        }
    }
    
    fn update_inputs(&mut self, node_id: usize) {
        for connection in self.connections.iter().filter(|c| c.dst_node_id == node_id) {
            if let Some(src_node) = self.operators.get(&connection.src_node_id) {
                if let Some(src_port) = src_node.get_output_port(&connection.src_port_id) {
                    let src_port_value = src_port.borrow().value;
                    if let Some(dst_node) = self.operators.get_mut(&node_id) {
                        if let Some(dst_port) = dst_node.get_input_port(&connection.dst_port_id) {
                            dst_port.borrow_mut().value = src_port_value;
                            println!("Value updated: node {} input port {} now {}",
                                     node_id, connection.dst_port_id, dst_port.borrow().value);
                        }
                    }
                }
            }
        }
    }
    
    fn topological_sort(nodes: &HashMap<usize, Box<dyn Operator>>, connections: &Vec<Connection>) -> Vec<usize> {
        let mut in_degree = HashMap::new();
        let mut zero_in_degree = Vec::new();
        let mut order = Vec::new();

        // 初始化入度表
        for node in nodes.keys() {
            in_degree.insert(*node, 0);
        }

        // 计算所有节点的入度
        for conn in connections {
            *in_degree.entry(conn.dst_node_id).or_insert(0) += 1;
        }

        // 找到所有入度为0的节点
        for (node_id, &degree) in &in_degree {
            if degree == 0 {
                zero_in_degree.push(*node_id);
            }
        }

        // Kahn算法主循环
        while let Some(node_id) = zero_in_degree.pop() {
            order.push(node_id);
            for conn in connections.iter().filter(|c| c.src_node_id == node_id) {
                let entry = in_degree.entry(conn.dst_node_id).or_default();
                *entry -= 1;
                if *entry == 0 {
                    zero_in_degree.push(conn.dst_node_id);
                }
            }
        }

        // 检查是否存在环
        if order.len() == nodes.len() {
            order
        } else {
            vec![]
        }
    }
}