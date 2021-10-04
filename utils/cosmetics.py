import json
import requests

MAPPINGS = {
    "AthenaEmoji": "AthenaDance",
    "AthenaToy": "AthenaDance",
    "AthenaSpray": "AthenaDance",
    "AthenaPet": "AthenaBackpack",
    "AthenaPetCarrier": "AthenaBackpack"
}

data = []
file = open("cosmetics.json", "w")
season = int(input("Introduction: "))
items = requests.get("https://fortnite-api.com/v2/cosmetics/br").json()

for item in items["data"]:
    if "Default" in item["id"] or item["introduction"] and item["introduction"]["backendValue"] <= season:
        item_type = item["type"]["backendValue"]
        
        if item_type in MAPPINGS:
            item_type = MAPPINGS[item_type]
        
        variants = []
        if item["variants"]:
            variants = list(map(lambda v: {
                "channel": v["channel"],
                "options": list(map(lambda o: o["tag"], v["options"]))
            }, item["variants"]))
        
        data.append({
            "type": item_type,
            "id": item["id"].lower(),
            "variants": variants
        })

file.write(json.dumps(data, indent=4))