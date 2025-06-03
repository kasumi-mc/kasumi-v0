import os
import json
from typing import Any, List, Dict

# Map each registry directory to its correct protocol registry name
REGISTRIES = {
    # "data/minecraft/banner_pattern": "minecraft:banner_pattern",
    # "data/minecraft/chat_type": "minecraft:chat_type",
    # "data/minecraft/damage_type": "minecraft:damage_type",
    # "data/minecraft/dimension_type": "minecraft:dimension_type",
    # "data/minecraft/painting_variant": "minecraft:painting_variant",
    # "data/minecraft/trim_material": "minecraft:trim_material",
    # "data/minecraft/trim_pattern": "minecraft:trim_pattern",
    # "data/minecraft/wolf_variant": "minecraft:wolf_variant",
    "data/minecraft/worldgen/biome": "minecraft:worldgen/biome",
}

def generate_registry_entries(root: str, registry_name: str) -> Dict[str, Any]:
    """
    Recursively scans root for .json files, generates protocol-correct entries for the registry.
    """
    entries: List[tuple[str, str]] = []
    for dirpath, _, filenames in os.walk(root):
        for filename in filenames:
            if not filename.endswith(".json"):
                continue
            relative_path = os.path.relpath(os.path.join(dirpath, filename), root).replace('\\', '/')
            # Do not repeat the registry name in the entry name!
            identifier = f"minecraft:{relative_path[:-5]}"
            entries.append((identifier, os.path.join(dirpath, filename)))
    entries.sort(key=lambda x: x[0])  # Alphabetical
    # Now, load JSON and build entry list
    elements = {}
    for idx, (identifier, file_path) in enumerate(entries):
        with open(file_path, "r", encoding="UTF-8") as entry_file:
            element = json.load(entry_file)
        print(element)
        print(identifier)
        elements[identifier] = element
    return elements

def build_registry_dict(registries: dict) -> dict:
    """
    Build the full registry structure for all registries.
    """
    result = {}
    for folder, registry_name in registries.items():
        # If the folder does not exist, skip it (makes the script robust)
        if not os.path.isdir(folder):
            print(f"Warning: {folder} does not exist, skipping.")
            continue
        print(f"Processing {registry_name} from {folder}")
        elements = generate_registry_entries(folder, registry_name)
        result[registry_name] = elements
    return result

if __name__ == "__main__":
    registry = build_registry_dict(REGISTRIES)
    with open("registry.json", "w", encoding="UTF-8") as f:
        json.dump(registry, f, indent=2)
    print("Done. Wrote registry.json.")
