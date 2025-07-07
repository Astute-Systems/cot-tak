#!/usr/bin/env python3
import xml.etree.ElementTree as ET

def parse_xml(file_path):
    tree = ET.parse(file_path)
    root = tree.getroot()
    return root

def generate_rust_enum(root):
    rust_code = "#[derive(PartialEq)]"
    rust_code = "enum CoTType {\n"
    for cot in root.findall('cot'):
        cot_value = cot.get('cot') or cot.get('zot')
        full = cot.get('full')
        desc = cot.get('desc')
        if (full != None):
            rust_code += f"    /// {desc}\n"
            # Check full is not empty 
            if ( full != None) & (full != "MAYDAYMESSAGE") & (full != "GROUND") & (full != "OTHER"):
                full = full.replace('/', '_')
                full = full.replace(' ', '')
                full = full.replace(',', '_')
                full = full.replace('(', '_')
                full = full.replace(')', '_')
                full = full.replace('+', '_')
                full = full.replace('-', '_')
                # Convert snake case to camel case 
                full = ''.join(word.title() for word in full.split('_'))
                
                rust_code += f"    {full},\n"
    rust_code += "}\n"

    # Now create the array of enum to cot string cot_value
    rust_code += "\n"
    rust_code += "// Emun CoTType and string as tuple"
    rust_code += "static COT_STRINGS: &[(CoTType, &str)] = &[\n"
    for cot in root.findall('cot'):
        cot_value = cot.get('cot') or cot.get('zot')
        full = cot.get('full')
        desc = cot.get('desc')
        if ( full != None) & (full != "MAYDAYMESSAGE") & (full != "GROUND") & (full != "OTHER"):
            # Check full is not empty 
            full = full.replace('/', '_')
            full = full.replace(' ', '')
            full = full.replace(',', '_')
            full = full.replace('(', '_')
            full = full.replace(')', '_')
            full = full.replace('+', '_')
            full = full.replace('-', '_')
            # Convert snake case to camel case 
            full = ''.join(word.title() for word in full.split('_'))

            rust_code += f"    (CoTType::{full}," 
            cot_value = cot.get('cot') or cot.get('zot')
            rust_code += f" \"{cot_value}\"),\n"
    rust_code += "];\n"


    return rust_code

def save_rust_code(file_path, rust_code):
    with open(file_path, 'w') as file:
        file.write(rust_code)

if __name__ == "__main__":
    # Print file locations

    print ("XML file path: /home/newman/repos/CotX/docs/CoTtypes.xml")
     
    xml_file_path = "/home/newman/repos/CotX/docs/CoTtypes.xml"
    rust_file_path = "/home/newman/repos/CotX/docs/CoTtypes.rs"
    
    root = parse_xml(xml_file_path)
    rust_code = generate_rust_enum(root)
    save_rust_code(rust_file_path, rust_code)
