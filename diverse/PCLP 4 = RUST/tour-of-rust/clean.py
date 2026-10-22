import os

def remove_last_three_chars(input_str):
    if len(input_str) >= 3:
        modified_str = input_str[:-3]
        return modified_str
    else:
        return None

def remove_rust_files():

    # Get the current directory
    cwd = os.getcwd()

    # List all files in the directory
    files = os.listdir(cwd)

    # Iterate through the files
    for file in files:
        # Check if the file ends with '.rs'
        if file.endswith('.rs'):
            # Construct the full path
            rust_exe = remove_last_three_chars(file)
            file_path = os.path.join(cwd, rust_exe)
            
            # Remove the file
            try:
                os.remove(file_path)
                print(f"Removed: {file_path}")
            except Exception as err:
                continue;


if __name__ == "__main__":
    remove_rust_files()
