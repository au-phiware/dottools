macro_rules! parse {
    ($name:ident, $input:expr, $graph:expr) => {
        #[test]
        fn $name() {
            let input = $input;
            let g = input.parse::<Graph>().unwrap();
            println!("{:?}", g);
            let output = g.to_string();
            let mut v = g.all_edges().collect::<Vec<(Node, Node, &Edge)>>();
            let mut g = $graph;
            v.sort_by_key(|e| (e.0, e.1));
            g.sort_by_key(|e| (e.0, e.1));
            assert_eq!(&v[..], &g[..]);
            assert_eq!(clean_string(input.trim_end()), output);
        }
    };
}

mod diagonal;
mod fuzzer;
mod horizontal;
mod node;
mod vertical;
