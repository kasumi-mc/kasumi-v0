import json
import os
from typing import Any

REGISTRIES = {
    "data/minecraft/cat_variant": "minecraft:cat_variant",
    "data/minecraft/chicken_variant": "minecraft:chicken_variant",
    "data/minecraft/cow_variant": "minecraft:cow_variant",
    "data/minecraft/frog_variant": "minecraft:frog_variant",
    "data/minecraft/painting_variant": "minecraft:painting_variant",
    "data/minecraft/pig_variant": "minecraft:pig_variant",
    "data/minecraft/wolf_sound_variant": "minecraft:wolf_sound_variant",
    "data/minecraft/wolf_variant": "minecraft:wolf_variant",
    "data/minecraft/worldgen/biome": "minecraft:worldgen/biome"
}


def generate_registry_entries(root_path: str) -> dict[str, Any]:
    entries: list[tuple[str, str]] = []
    for dirpath, _, filenames in os.walk(root_path):
        for filename in filenames:
            if not filename.endswith(".json"):
                continue
            
            relative_path = os.path.relpath(os.path.join(dirpath, filename), root_path).replace('\\', '/')
            entry_identifier = f"minecraft:{relative_path[:-5]}"

            print(f"* New entry found: {entry_identifier} (path: {relative_path})")
            entries.append((entry_identifier, os.path.join(dirpath, filename)))
    entries.sort(key=lambda x: x[0])
    
    elements = {}
    for (identifier, file_path) in entries:
        with open(file_path, "r", encoding="UTF-8") as entry_file:
            element = json.load(entry_file)
        elements[identifier] = element
    
    return elements


def build_registries_dict(registries: dict[str, str]) -> dict:
    result = {}
    for folder, registry_name in registries.items():
        if not os.path.isdir(folder):
            print(f"Warning: {folder} doesn't exist, skipping...")
            continue
        print(f"Now processing {registry_name} from {folder}")

        elements = generate_registry_entries(folder)
        result[registry_name] = elements
        print(f"* Collected {len(elements)} elements")
    return result


if __name__ == "__main__":
    registries = build_registries_dict(REGISTRIES)
    with open("new_registry.json", "w+", encoding="UTF-8") as output_file:
        json.dump(registries, output_file, indent=2)
    print(f"Done. Wrote `new_registry.json` with {len(registries)} entries")
