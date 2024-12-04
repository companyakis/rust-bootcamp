curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

rustup target add wasm32-unknown-unknown

sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"

Note: Confirm the IC SDK has been installed (you may need to open a new terminal window):

dfx --version

![image](https://github.com/user-attachments/assets/e8c0b7cd-35b7-4e0e-a86a-80564e64d442)

