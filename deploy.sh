# Build the project
# cargo build --release --target wasm32-unknown-unknown --package stringly_backend

# Check if the build was successful
if [ $? -ne 0 ]; then
    echo "Build failed. Exiting."
    exit 1
fi

# Extract candid interface from the WASM file
candid-extractor /home/harsh/rs-token/target/wasm32-unknown-unknown/release/rs_token_backend.wasm > src/rs-token-backend/rs-token-backend.did

# Check if candid extraction was successful
if [ $? -ne 0 ]; then
    echo "Candid extraction failed. Exiting."
    exit 1
fi

echo "Deployment script completed successfully."