import "./custom.css";

import init, {
  getCharacterSheetJsonSchema,
} from "character-sheet-renderer/engine/character_sheet_js.js";
import { createRendererFromJson } from "character-sheet-renderer";

async function run() {
  const EXAMPLE_JSON = `{
        "userValues": {
            "strength": {
                "number": 10
            },
            "speed": {
                "dice": {
                    "dice": [{"amount": 2, "sides": 4, "modifiers": []}],
                    "bonus": 2
                }
            }
        },
        "activeFeatures": [
            {
                "name": "Character",
                "description": "Every player character has a few features.",
                "source": "Basic Rules",
                "features": [
                    {
                        "name": "attributes",
                        "description": "Every player character has 4 basic attributes: strength, speed, resilience, and intelligence.",
                        "baseType": "attributes",
                        "definitions": [
                            {"name": "strength",    "selector": {"identifier": "highest", "arguments": []}, "limiters": [{"identifier": "min", "arguments": ["0"]}]},
                            {"name": "speed",       "selector": {"identifier": "highest", "arguments": []}, "limiters": [{"identifier": "min", "arguments": ["0"]}]},
                            {"name": "resilience",  "selector": {"identifier": "highest", "arguments": []}, "limiters": [{"identifier": "min", "arguments": ["0"]}]},
                            {"name": "intelligence", "selector": {"identifier": "highest", "arguments": []}, "limiters": [{"identifier": "min", "arguments": ["0"]}]}
                        ],
                        "modifiers": []
                    },
                    {
                        "name": "defense",
                        "description": "Your defense is based on your speed and resilience.",
                        "baseType": "combat_stats",
                        "definitions": [],
                        "modifiers": [
                            {"property": "defense", "value": {"script": {"script":"$speed + $resilience", "dependencies": ["speed", "resilience"]}}}
                        ]
                    }
                ]
            },
            {
                "name": "Gambler",
                "description": "Your character is a gambler. They have luck-related abilities.",
                "source": "Luck be a Bet",
                "features": [
                    {
                        "name": "gambler_attributes",
                        "description": "Gamblers have an additional basic attribute called luck.",
                        "baseType": "attributes",
                        "definitions": [
                            {"name": "luck", "selector": {"identifier": "highest", "arguments": []}, "limiters": [{"identifier": "min", "arguments": ["0"]}]}
                        ],
                        "modifiers": []
                    },
                    {
                        "name": "gambler_lucky_defense",
                        "description": "A gambler's luck protects them from harm.",
                        "baseType": "combat_stats",
                        "definitions": [],
                        "modifiers": [
                            {"property": "defense", "value": {"script": {"script":"$@+$luck", "dependencies": ["luck"]}}}
                        ]
                    }
                ]
            }
        ],
        "inactiveFeatures": []
    }`;

  await init();

  const jsonSchema = getCharacterSheetJsonSchema();
  console.log(JSON.stringify(JSON.parse(jsonSchema), null, 2));

  const renderer = createRendererFromJson(EXAMPLE_JSON);
  console.log({ renderer });
  renderer.bindToDom(document.getElementById("auto-bind-root"));
}
run();
