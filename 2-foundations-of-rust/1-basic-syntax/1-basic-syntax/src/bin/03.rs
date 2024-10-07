fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let mut mx = input[0];
    let mut mn = input[1];
    for ele in input {
        if ele > mx {
            mx = ele
        } else if ele < mn {
            mn = ele
        }
    }

    println!("{} is largest and {} is smallest", mx, mn);
}
