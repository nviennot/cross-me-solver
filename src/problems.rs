// 11us to solve the 10x10 grid
pub fn def_2_29() -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let rows = vec![
        vec![2,4,2],
        vec![6],
        vec![5,4],
        vec![2,2],
        vec![1,1,1,1],

        vec![1,1],
        vec![1,4,1],
        vec![1,2,1],
        vec![1,1],
        vec![1,6,1],
    ];

    let cols = vec![
        vec![1,1,2,1],
        vec![1,4,1],
        vec![3,1],
        vec![3,1,1,1],
        vec![3,2,1],

        vec![2,2,1],
        vec![3,1,1,1],
        vec![3,1],
        vec![1,4,1],
        vec![1,1,2,1],
    ];

    (rows, cols)
}

// 96ms
pub fn def_4_1() -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let rows = vec![
        vec![4],
        vec![5],
        vec![2,3],
        vec![3,3,1],
        vec![3,3,3],

        vec![3,3,3],
        vec![3,3,2],
        vec![3,6],
        vec![3,4],
        vec![6],

        vec![1,5],
        vec![3,6],
        vec![5,3],
        vec![3,3],
        vec![2],
    ];

    let cols = rows.clone();
    (rows, cols)
}

// 170ms to solve this 40x20
pub fn def_5_392() -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let rows = vec![
        vec![7,3],
        vec![3,2,5,2],
        vec![2,2,2,1,5],
        vec![1,2,6,7],
        vec![3,1,3,6,6],

        vec![1,2,1,1,5,2,4],
        vec![2,1,2,4,6,4,4],
        vec![3,4,2,1,6,3],
        vec![1,5,2,2,4,3],
        vec![2,1,4,2,1,5,4,2],

        vec![6,1,4,3,6,4,2],
        vec![8,2,3,4,7,3,2],
        vec![8,3,4,7,3,1],
        vec![9,1,2,5,7,3],
        vec![9,1,3,4,7,2,2],

        vec![11,1,2,5,7,1,2],
        vec![5,2,1,2,2,5,5,1,2],
        vec![2,1,1,1,2,5,3,2],
        vec![2,2,3,2,2,2],
        vec![3,4,2],
    ];

    let cols = vec![
        vec![1],
        vec![2],
        vec![2],
        vec![1,3],
        vec![2,4],

        vec![2,4],
        vec![7],
        vec![2,7,1],
        vec![1,1,8],
        vec![1,7],

        vec![2,5],
        vec![2,3],
        vec![1,3],
        vec![2,7,5],
        vec![2,4,1,2],

        vec![1,5,2,1],
        vec![2,8,1,2],
        vec![1,10,2],
        vec![1,6,3],
        vec![1,2,3,3,2],

        vec![1,3,1,7,1],
        vec![1,1,1,1,8],
        vec![2,1,3,8],
        vec![6,2,6],
        vec![7,3,4],

        vec![1,4,6,3],
        vec![2,4,8,1],
        vec![2,2,1,9],
        vec![3,1,1,9],
        vec![1,1,3,9],

        vec![2,4,8],
        vec![4,5,4,1],
        vec![4,7,1,1],
        vec![4,8,1],
        vec![5,6,1],

        vec![6,1,2,2],
        vec![9,1,2],
        vec![6,4,1],
        vec![2,2],
        vec![1],
    ];

    (rows, cols)
}
