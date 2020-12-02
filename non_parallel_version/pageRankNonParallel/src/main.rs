use rand::seq::IteratorRandom;
use std::time::{Duration, Instant};
#[derive(Clone)]
struct Node<'a> { // all Nodes should be in the same lifetime
    towards: Vec<&'a i32>, // Node is pointing towards
    from: Vec<&'a i32>,    // Node is is getting pointed from
    value: i32,         // it has a name (value), I have tried to make it a String but rust says "strings do not pocess the copy property"
    pagerank: f32,      // the page rank of the node
}

fn main(){
    /*//==============================simple graph=====================================
    let mut _a = Node {
        towards: vec![],
        from: vec![],
        value: 1,
        pagerank: 0.0,
    };
    let mut _b = Node { 
        towards: vec![],
        from: vec![],
        value: 2,
        pagerank: 0.0,
    };
    let mut _c = Node { 
        towards: vec![],
        from: vec![],
        value: 3,
        pagerank: 0.0,
    };
    let mut _d = Node { 
        towards: vec![],
        from: vec![],
        value: 4,
        pagerank: 0.0,
    };
    let mut _e = Node { 
        towards: vec![],
        from: vec![],
        value: 5,
        pagerank: 0.0,
    };
    let mut _f = Node { 
        towards: vec![],
        from: vec![],
        value: 6,
        pagerank: 0.0,
    };
    let mut _g = Node { 
        towards: vec![],
        from: vec![],
        value: 7,
        pagerank: 0.0,
    };

    _a.towards = vec![&_b.value];
    _a.from = vec![&_c.value];

    _b.towards = vec![&_d.value, &_a.value, &_c.value, &_e.value, &_f.value]; // it is very obvious that b is the most important
    _b.from = vec![&_a.value,&_c.value, &_d.value, &_e.value, &_f.value];

    _c.towards = vec![&_a.value,&_b.value,&_d.value];
    _c.from = vec![&_a.value, &_d.value];

    _d.towards = vec![&_c.value];
    _d.from = vec![&_c.value,&_b.value];

    _e.towards = vec![&_f.value];
    _e.from = vec![&_f.value];

    _f.towards = vec![&_e.value];
    _f.from = vec![&_e.value];

    _g.towards = vec![]; // it is very obvious that g is the least important
    _g.from = vec![];    // infact it has no connections. I only added it to see what happens ┌༼◉ل͟◉༽┐

    // puts all the nodes in 1 place
    let mut _simple_graph = vec![_a.clone(),_b.clone(),_c.clone(),_d.clone(),_e.clone(),_f.clone(),_g.clone()];
    */
    let number_of_nodes : Vec<i32> = (0..100).collect();
    let big_graph = create_graph(100, &number_of_nodes);
    //==============================================================================
    //println!("{:?}",page_rank(&_simple_graph, 1000));
    println!("non parallel");
    let now = Instant::now();
    page_rank(&big_graph,100);
    println!("{}", now.elapsed().as_millis());
}


/**
 * function that takes the number of nodes you want.
 * then a vector of i32 which is basically the values of the nodes. (names ofcourse being the numbers)
 */
fn create_graph<'a>(nodes: i32, number_of_nodes: &Vec<i32>) -> Vec<Node> {
    let mut random_iterator = rand::thread_rng();
    let mut node_vector: Vec<Node> = vec![];
    let mut number_of_nodes_towards: Option<i32>;
    let mut number_of_nodes_from: Option<i32>;
    for i in 0..number_of_nodes.len() {
        // get random number of nodes
        number_of_nodes_towards = (1..nodes).choose(&mut random_iterator); // randomly pick a number between 1 and nodes
        number_of_nodes_from = (1..nodes).choose(&mut random_iterator);
        node_vector.push(
            // use the numbers picked to pick "number_of_towards" of random nodes
            // same for from
            // choose_multiple should not contain duplicates (unless stack overflow lied to me)
            Node {
                towards: (0..nodes).choose_multiple(&mut random_iterator, number_of_nodes_towards.unwrap() as usize)
                .into_iter().map(|x| &number_of_nodes[x as usize]).collect(), // randomly choose a number of nodes, no duplicates
                from: (0..nodes).choose_multiple(&mut random_iterator, number_of_nodes_from.unwrap() as usize)
                .into_iter().map(|x| &number_of_nodes[x as usize]).collect(), // randomly choose a number of nodes, no duplicates
                value: number_of_nodes[i],
                pagerank: 0.0,
            }
        );
    }
    return node_vector;
}

/**
 * function takes in a vector of nodes and a number n for iterations
 * returns the values (names) in order of rank
*/
fn page_rank(_object: &Vec<Node>, n: i32) -> Vec<i32>{
    // the equation is
    // for PageRank of A in iteration i+1 = 
    //         sum of { n = nodes pointing to A}
    //         ( PageRank of n in iteration i / number of nodes n points to )
    let mut temp_object: Vec<Node> = _object.clone(); // our current iteration
    for i in 0.._object.len(){
        temp_object[i].pagerank = 1.0/(_object.len() as f32);
    }
    let mut _prev_iter: Vec<Node>; // previous iteration
    // ========================================DO PAGE RANK====================================================
    let mut index: i32;
    let mut temp_value: f32;
    for _iteration in 0..n {
        _prev_iter = temp_object.clone(); // set previous iteration
        index = 0; // used to index the objects in _to_compare (to set the objects in the current index)
        for node in _object {
            temp_value = do_page_rank(&node, _object, &_prev_iter); // calculate using the formulae
            temp_object[index as usize].pagerank = temp_value; // reset the value
            index = index + 1;
        }
    }
    // ========================================================================================================
    let mut _to_compare : Vec<f32> = vec![0.0; temp_object.len()]; // create a vector which will hold the pagerank of each node
    for pageranks in 0.._to_compare.len() {
        _to_compare[pageranks] = temp_object[pageranks].pagerank; // place the page ranks of eachnode in position of _object
    }
    let mut correct_order: Vec<f32> = _to_compare.clone(); // get the correct order of the page ranks
    correct_order.sort_by(|b, a| a.partial_cmp(b).unwrap()); // aka decreasing order, higher the number, the more important
    let mut to_return = vec![0;_object.len()]; // finally create a vector to return the names/values of each node (in correct order)

    // uncomment the for fun to see what the original names passed in were (most likely 1,2,3,4,5,6... or 0,1,2,3,4,5....)
    let mut for_fun = to_return.clone();
    let mut all_indexes = vec![-1; _object.len()]; // all indexes will hold all the indexes of the graph.
    // to prevent duplicates, if 2 nodes have the same pagerank. it will return the first one twice. we dont want that.
    for order in 0..correct_order.len() {
        /*
        so. (I confused myself reading my own code so Im gonna write a full essay)
        correct_order contains the page ranks in correct order (Vec<f32>)
        _to_compare contains the page ranks (Vec<f32>) in the same order as the objects vector (Vec<Node>).
        so the index of the _to_compare[index] == _object[index].pagerank
        so finding the index of the correct_order in _to_compare, then using it to get the node in _object. 
        we can retrieve the nodes name/value
        */
        let index = get_index(&correct_order[order], &_to_compare, &all_indexes);
        to_return[order] = temp_object[index as usize].value;
        //for_fun[order] = temp_object[order].value;
        all_indexes[order] = index;
    }
    //println!("{:?}", for_fun);
    return to_return;
}
// there is probably a better way todo this but alas monkey brain has got the better of me
// check if the nodes index in the correct order already exists in to_return
// if not. add it. if yes. ignore it.
fn get_index(number: &f32, vector: &Vec<f32>, to_return: &Vec<i32>) -> i32{
    for i in 0..vector.len() {
        if number == &vector[i] {
            if !to_return.iter().any(|x| *x == i as i32){
                return i as i32;
            }
        }
    }
    println!("this is run");
    return -10 as i32; // should never be reach but this is here to not make it crash
}

// given a value, find that objects index in object
fn find_the_node(v: i32, _object: &Vec<Node>) -> i32{
    for i in 0.._object.len() {
        if _object[i].value == v {
            return i as i32;
        }
    }
    return -10; // should never be reach but this is here to not make it crash
}

fn do_page_rank(node: &Node, _object: &Vec<Node>, _prev_iter: &Vec<Node>) -> f32{
    let mut to_return: f32 = 0.0; // used to tally up the answer
    let mut index;
    for &node in &node.from { // The node.from is a int vector. Which is the index of the object in the objects vector
        index = find_the_node(*node, _object); // fixes the issue of if you wanted to name the objects something else
        to_return = to_return + (_prev_iter[index as usize].pagerank as f32) / (_object[index as usize].from.len() as f32)
    }
    return to_return;
}