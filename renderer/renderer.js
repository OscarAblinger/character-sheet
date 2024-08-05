// This is the actual renderer library
import { createFromJson, findMinimumRequiredUserValues, getAsJson, setUserValueFromJson } from './engine/character_sheet_js.js';

// TODO: implement binding
// naming scheme:
// - data-cs-bind-<name>: 
//      for value on inputs: two way binding
//      for objects or object arrays: use template either in value or data-cs-to
//      everything else: error
// - data-cs-bind-<name>-to: binds value to specified property of the DOM node

const knownNames = {};

const idAdjectives = ['radiant', 'mysterious', 'vibrant', 'serene', 'whimsical', 'tenacious', 'luminous', 'eccentric', 'dynamic', 'tranquil'];
const idNouns = ['phoenix', 'galaxy', 'cascade', 'enigma', 'horizon', 'odyssey', 'echo', 'labyrinth', 'mirage', 'summit'];

function createRandomId() {
    if (Object.keys(knownNames).length > idAdjectives.length * idNouns.length * 10) {
        // what are you doing?
        throw "All random ids are used up. You really should only have one character sheet per renderer anyways";
    }
    function r(max) {
        return Math.floor(Math.random() * max);
    }

    return `${idAdjectives[r(idAdjectives.length)]}-${idNouns[r(idNouns.length)]}-${r(99)}`
}

export function createRendererFromJson(json) {
    const name = createRandomId();
    createFromJson(name, json)
    return new CharacterSheetRenderer(name)
}

function value2text(value) {
    return value.number ?? dice2text(value.dice);
}

function dice2text({ dice, bonus}) {
    let text = "";

    for (const {amount, sides, modifiers} of dice) {
        if (text !== "" && amount >= 0) {
            text += "+";
        }

        text += `${amount}d${sides}`
        for (const modifier of modifiers) {
            text += `${modifier}`
        }
    }

    if (bonus !== 0) {
        text += bonus > 0 ? `+${bonus}` : `${bonus}`;
    }
    return text;
}

function text2Value(text) {
    text = text.trim();
    if (text === "") {
        return null;
    }
    const number = new Number(text);
    if (!isNaN(number)) {
        return {number};
    }
    return {dice: parseDice(text)};
}

function parseDice(text) {
    const dice = {
        dice: [],
        bonus: 0,
    };

    text = text.replace(/\s+/g, "");

  	let oldText = text;
    while (text.length > 0) {
        // substring necessary for starting + or -
        let i = text.substring(1).search(/[+-]/);
        i = i === - 1 ? text.length : i + 1; // +1 from the substring(1)
        const part = text.substring(0, i);
        text = text.substring(i);

        if (part.includes("d")) {
            const [amount, sides] = part.split("d");
            dice.dice.push({amount: new Number(amount), sides: new Number(sides), modifiers: []});
        } else {
            dice.bonus += new Number(part);
        }
        if (oldText === text) throw {text, dice};
        oldText = text;
    }

    return dice;
}

function newInputBinder(csRenderer, node, selector, updateBaseType) {
    const binder = csRenderer.createBinder(({newCS}) => {
        // no check for changes as that would just be slower than just doing it
        const value = selector(newCS);
        node.type = value.number ? "number" : "text";
        node.value = value.number ?? dice2text(value.dice);
    });
    node.addEventListener("change", () => {
        binder.update({...updateBaseType, value: text2Value(node.value)});
    });
    return binder;
}

function bindValue2Way(csRenderer, binders, node, selector, updateBaseType) {
    if (node.tagName === "INPUT") {
        binders.push(newInputBinder(csRenderer, node, selector, updateBaseType));
    } else {
        throw {message: `two-way binding is only supported on <input> nodes.`, node};
    }
}

function bindTo(node, propertyName, value) {
    if (propertyName.startsWith("#")) {
        if (propertyName === "#text-before" || propertyName === "#text") {
            if (node.firstChild.nodeType === Node.TEXT_NODE) {
                node.firstChild.textContent = value;
            } else {
                node.prepend(document.createTextNode(value));
            }
        } else if (propertyName === "#text-after") {
            if (node.lastChild.nodeType === Node.TEXT_NODE) {
                node.lastChild.textContent = value;
            } else {
                node.append(document.createTextNode(value));
            }
        }
    } else {
        node[propertyName] = value;
    }
}

function bindValueTo(csRenderer, binders, node, datasetKey, selector) {
    binders.push(csRenderer.createBinder(({newCS}) => {
        const value = selector(newCS);
        bindTo(node, node.dataset[datasetKey], value2text(value));
    }));
}

function low(str) {
    return str.charAt(0).toLowerCase() + str.slice(1);
}

function findSpecificUserValueBindings(node) {
    // this is likely a good place for future optimization
    return [...node.querySelectorAll("*")]
        .map(node => {
            return {
                node,
                set: Object.keys(node.dataset).filter(k => k.startsWith("csBindUserInput")),
            };
        })
        .filter(({set}) => set.length)
        .flatMap(({node, set}) => {
            return set
                .filter(datasetKey => datasetKey != "csBindUserInputs")
                .map(datasetKey => {
                return {
                    node,
                    datasetKey,
                    userValueKey: low(datasetKey.replace("csBindUserInput", "").replace(/To$/, "")),
                };
            });
        });
}

function bindList2Way(csRenderer, binders, node, template, selector) {
    binders.push(csRenderer.createBinder(function({newCS}) {
        const documentFragments = [];
        const newValues = selector(newCS);

        if (JSON.stringify(this.oldValues) === JSON.stringify(newValues)) {
            console.log("no changes - no need to update");
            return;
        }


        for (const {name, value} of newValues) {
            const instance = template.content.cloneNode(true);
            documentFragments.push(instance);

            for (const nameTo of instance.querySelectorAll(`[data-cs-bind-name-to]`)) {
                bindTo(nameTo, nameTo.dataset.csBindNameTo, name);
            }
            for (const valueTo of instance.querySelectorAll(`[data-cs-bind-value-to]`)) {
                bindTo(valueTo, valueTo.dataset.csBindNameTo, value2text(value));
            }
            for (const value2way of instance.querySelectorAll(`[data-cs-bind-value]`)) {
                value2way.type = value.number ? "number" : "text";
                value2way.value = value2text(value);
                value2way.addEventListener("change", () => {
                    this.update({ type: "user-input", property: name, value: text2Value(value2way.value)});
                });
            }
        }

        const newChildren = documentFragments.flatMap(instance => [...instance.childNodes]);

        if (this.oldInstances != null) {
            const len = Math.max(this.oldInstances.length, newChildren.length);
            for (let i = 0; i < len; i++) {
                if (this.oldInstances.length < i) {
                    node.appendChild(newChildren[i]);
                } else {
                    this.oldInstances[i].replaceWith(newChildren[i]);
                }
            }
        } else {
            newChildren.forEach(instance => node.appendChild(instance));
        }
        this.oldValues = newValues;
        this.oldInstances = newChildren;
    }));
}

function createAutomaticDOMBinders(csRenderer) {
    const binders = [];

    // user input bindings
    const userInputBindings = findSpecificUserValueBindings(csRenderer.rootNode);
    for (const {datasetKey, userValueKey, node} of userInputBindings) {
        if (datasetKey.endsWith("To")) {
            bindValueTo(csRenderer, binders, node, datasetKey, cs => cs.userValues[userValueKey]);
        } else {
            bindValue2Way(csRenderer, binders, node, cs => cs.userValues[userValueKey], {type: "user-input", property: userValueKey});
        }
    }
    for (const userInputsNode of csRenderer.rootNode.querySelectorAll("[data-cs-bind-user-inputs]")) {
        const templateName = userInputsNode.dataset.csBindUserInputs;
        const template = document.getElementById(templateName);
        if (template == null) {
            throw { message:`Could not find template with id ${templateName}.`, userInputsNode};
        }
        bindList2Way(csRenderer, binders, userInputsNode, template,
            cs => Object.entries(cs.userValues).map(([name, value]) => { return { name, value }; })
        );
    }
    // todo: feature set bindings
}

export class CharacterSheetRenderer {
    constructor(name) {
        this.name = name;
        this.rootNode = null;
        // the CharacterSheet object
        this.currentCS = null;
        // individual binders that may have to be updated
        this.binders = [];
    }

    /**
     * Binds this character sheet to the given DOM node.
     * It look at the node and its children to finde a 'data-cs-root' attribute
     * and begin binding from there.
     * Will immediately update the DOM and keep it updated with changes.
     */
    bindToDom(domElement) {
        if (this.rootNode != null) {
            throw "Renderer is already bound to a DOM element";
        }

        if (domElement.dataset.csRoot != null) {
            this.rootNode = domElement;
        } else {
            this.rootNode = domElement.querySelector('[data-cs-root]');
        }
        if (this.rootNode == null) {
            throw "No root node found for character sheet renderer";
        }

        if (this.currentCS == null) {
            createAutomaticDOMBinders(this);
            this.synchronize();
        } else {
            // this renderer was already synchronized before
            const newBinders = createAutomaticDOMBinders(this);
            newBinders.forEach(binder => binder.render({oldCS: null, newCS: this.currentCS}));
        }
    }
    synchronize() {
        const newCS = JSON.parse(getAsJson(this.name));
        this.binders.forEach(binder => binder.render({oldCS: this.currentCS, newCS}));
        this.currentCS = newCS;
    }

    /**
     * Creates and registers a new binder with the given render function.
     * If you want to stop the binder from being called, you need to provide it to `unregisterBinder`.
     *
     * The render function gets the old and new character sheets as its parameters and it's the job of
     * the render function to check whether there were any relevant changes if necessary.
     * The exact argument looks like {current: CharacterSheet, new: CharacterSheet}
     * For the initial render, current will be null. But renderers added after the initial render will
     * only be called on changes afterwards, so you might want to call them after registering them:
     * 
     *   const binder = cs.createBinder(renderFunction);
     *   binder.render({oldCS: null, newCS: cs.currentCS});
     *
     * It also has an update function that you can call when a value is updated.
     * For read-only binding, you never have to call it. But for two-way binding, you'll have to call
     * it every time the value changes. For performance reasons, it usually makes sense to first check
     * if there is an actual change in the value.
     * The argument can be an array of changes or one change object in the form of e.g.
     *   { type: "user-input", property: "key", value: { number: 42 } }
     */
    createBinder(render) {
        const binder = {
            render,
            update: (changes) => {
                if (!Array.isArray(changes)) {
                    changes = [changes];
                }
                for (const change of changes) {
                    if (change.type == "user-input") {
                        const success = setUserValueFromJson(this.name, change.property, JSON.stringify(change.value));
                        if (success !== true) {
                            console.error(`Failed to set user value: ${success}`, change);
                        }
                    } else {
                        console.error(`Update type of ${change.type} is not supported`, change);
                    }
                }
                this.synchronize();
            }
        };
        this.binders.push(binder);
        return binder;
    }
    unregisterBinder(binder) {
        this.binders.remove(binder);
    }
}
