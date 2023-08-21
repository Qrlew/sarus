# Define the directory containing your protobuf files
PROTO_DIR = sarus_data_spec/protobuf

# Define the output directory for Rust code
RUST_OUT_DIR =  src/protobuf

# List of protobuf files in the directory
PROTO_FILES = $(wildcard $(PROTO_DIR)/*.proto)

# Generate Rust code
generate:
	protoc --rust_out=$(RUST_OUT_DIR) $(PROTO_FILES)

# Clean generated Rust files
clean:
	rm -rf $(RUST_OUT_DIR)/*

# Define the default target
.PHONY: generate_rust clean
.DEFAULT_GOAL := generate_rust