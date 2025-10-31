import copy
import glob
import json
import os
import subprocess

from fuzzomatic.tools.utils import detect_crate_name


def parse_item(index, it, path):
    functions = []

    if it["visibility"] != "public":
        return []

    if "module" in it["inner"]:
        module = it["inner"]["module"]
        module_name = it["name"]
        path.append(module_name)
        funcs = parse_module(index, module, path)
        functions.extend(funcs)
    elif "import" in it["inner"]:
        imp = it["inner"]["import"]
        funcs = parse_import(index, imp, path)
        functions.extend(funcs)
    elif "struct" in it["inner"]:
        struct = it["inner"]["struct"]
        struct_name = it["name"]
        path.append(struct_name)
        funcs = parse_struct(index, struct, path)
        functions.extend(funcs)
    elif "function" in it["inner"]:
        funcs = parse_function(index, it, path)
        functions.extend(funcs)

    return functions


def parse_function(index, it, path):
    functions = []
    if "function" in it["inner"] and it["visibility"] == "public":
        function_name = it["name"]
        function_inner = it["inner"]["function"]
        
        # Handle both old format (decl) and new format (sig)
        if "decl" in function_inner:
            # Old format: {"decl": {"inputs": [[name, type], ...]}}
            function_decl = function_inner["decl"]
            inputs = function_decl["inputs"]
            
            args = []
            for arg in inputs:
                argument_name = arg[0]
                extra = arg[1]
                if argument_name != "self":
                    arg_type = determine_arg_type(index, extra)
                    args.append(arg_type)
                else:
                    args.append("self")
        elif "sig" in function_inner:
            # New format: {"sig": {"inputs": [type, ...]}}
            function_sig = function_inner["sig"]
            inputs = function_sig["inputs"]
            
            args = []
            for arg_type_info in inputs:
                # In the new format, inputs are just types without names
                # We need to parse the type directly
                arg_type = determine_arg_type(index, arg_type_info)
                args.append(arg_type)
        else:
            # Unknown format, skip
            return functions
            
        functions.append((path, function_name, args))
    return functions


def determine_arg_type(index, extra):
    arg_type = "unknown"

    if "borrowed_ref" in extra:
        borrowed_ref = extra["borrowed_ref"]
        typ = borrowed_ref["type"]
        if "primitive" in typ:
            primitive_type = typ["primitive"]
            arg_type = f"&{primitive_type}"
        elif "array" in typ:
            array = typ["array"]
            if "type" in array:
                array_type = array["type"]
                if "len" in array:
                    array_length = array["len"]
                    if "primitive" in array_type:
                        array_primitive_type = array_type["primitive"]
                        arg_type = ("&array", array_primitive_type, array_length)
        elif "slice" in typ:
            slice = typ["slice"]
            if "primitive" in slice:
                primitive_type = slice["primitive"]
                arg_type = f"&[{primitive_type}]"
    elif "primitive" in extra:
        arg_type = extra["primitive"]
    if "resolved_path" in extra:
        resolved_path = extra["resolved_path"]
        path_id = resolved_path["id"]
        name = resolved_path["name"]
        try:
            _ = index[path_id]
        except KeyError:
            if name == "String":
                # Arg of type String, but not a custom String type
                arg_type = "String"

    return arg_type


def parse_struct(index, struct, path):
    functions = []
    impls = struct["impls"]
    for impl in impls:
        impl = index[impl]
        items = impl["inner"]["impl"]["items"]
        for item in items:
            item = index[item]
            funcs = parse_item(index, item, copy.deepcopy(path))
            functions.extend(funcs)

    return functions


def parse_module(index, module, path):
    functions = []
    for item in module["items"]:
        it = index[item]
        funcs = parse_item(index, it, copy.deepcopy(path))
        functions.extend(funcs)
    return functions


def parse_import(index, imp, path):
    functions = []
    ref = imp["id"]
    try:
        child_elem = index[ref]
        funcs = parse_item(index, child_elem, path)
    except KeyError:
        funcs = []
    functions.extend(funcs)
    return functions


def parse_cargo_doc_json(path):
    with open(path) as f:
        jso = json.loads(f.read())

    # get functions that take only one parameter and that are public
    root = jso["root"]
    index = jso["index"]

    # Check for both integer and string representations of the root key
    if root not in index and str(root) not in index:
        raise KeyError(f"The root key {root} does not exist in the index. Verify the JSON structure. {path}")

    # Use the appropriate key type
    root_elem = index[root] if root in index else index[str(root)]
    root_inner_items = root_elem["inner"]["module"]["items"]

    # Get the crate name from the root element to start the module path
    crate_name = root_elem.get("name", "")
    initial_path = [crate_name] if crate_name else []

    functions = []
    for elem in root_inner_items:
        # Ensure elem is treated as a string when accessing the index
        elem_key = str(elem) if elem not in index else elem
        e = index[elem_key]
        # Make a copy of initial_path for each iteration to avoid mutation
        funcs = parse_item(index, e, copy.deepcopy(initial_path))
        functions.extend(funcs)

    return functions


def generate_cargo_doc_json(codebase_dir, root_codebase_dir=None):
    cmd = [
        "cargo",
        "+nightly",
        "rustdoc",
        "--lib",
        "--",
        "--output-format",
        "json",
        "-Z",
        "unstable-options",
        "-A",
        "rustdoc::all",
    ]

    json_file_path = None

    try:
        subprocess.check_call(cmd, cwd=codebase_dir)
        target_root = codebase_dir
        if root_codebase_dir is not None:
            target_root = root_codebase_dir
        target = os.path.join(target_root, "target", "doc")
        crate_name = detect_crate_name(codebase_dir)
        json_file_path = os.path.join(target, f"{crate_name}.json")
        if os.path.exists(json_file_path):
            return json_file_path
        else:
            for f in glob.glob(f"{target}/*.json"):
                json_file_path = f
    except subprocess.CalledProcessError:
        print("Error: failed to generate cargo doc json")

    return json_file_path
