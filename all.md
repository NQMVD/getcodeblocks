> First line of quote
> Second line

This is a `code`.

```bash
#!/bin/bash

# Function to convert hex to RGB using the HEX2RGB command
convert_hex_to_rgb() {
    local hex_color=$1
    rgb_color=$(gum style --background="$hex_color" --foreground="#000000" "$hex_color")
    if [ $? -ne 0 ]; then
        echo "Error: Failed to convert $hex_color to RGB" >&2
        exit 1
    fi
    echo "$rgb_color"
}

# Read input from stdin
while IFS= read -r line; do
    # Use a regex to find all hex color codes in the line
    modified_line="$line"
    hex_colors=$(echo "$line" | grep -o -E '#[0-9a-fA-F]{6}')
    
    for hex_color in $hex_colors; do
        rgb_color=$(convert_hex_to_rgb "$hex_color")
        modified_line=$(echo "$modified_line" | sed "s/$hex_color/$rgb_color/g")
    done
    
    echo "$modified_line"
done
```

:octopus: :zap: :cat: = :heart:

This text is *emphasized*.

~~Scratch this~~.

This text is **strong**.

# h1

## h2

### h3

#### h4

##### h5

###### h6

---

![Image](https://charm.sh/logo.png).

This is a [link](https://charm.sh).

1. First Item
2. Second Item

- First Item
    - Nested List Item
- Second Item

3. 3 is first and numbered 3
4. 4 is second and numbered 4
10. ten is third and numbered 5

| Label  | Value | URL              |
| ------ | ----- | ---------------- |
| First  | foo   | https://charm.sh |
| Second | bar   | https://charm.sh |
| Label | Value | URL |
| :----- | :---: | ------: |
| First | foo | charm.sh |
| Second | bar | https://charm.sh |

- [x] Finished Task
- [ ] Outstanding Task
