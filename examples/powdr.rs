use tiny_keccak::{Hasher, Keccak};

fn main() {
    let input = b"Solidity";
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();

    // At this point, the state is all 0s
    hasher.state.keccak();

    // Add input and do full hashing
    hasher.update(input);
    hasher.finalize(&mut output);
}
