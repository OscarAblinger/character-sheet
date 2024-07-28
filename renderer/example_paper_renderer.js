function dom(type, classes, finish, ...children) {
    const el = document.createElement(type);
    if (classes) {
        for (const className of classes) {
            el.classList.add(className);
        }
    }
    if (finish) {
        finish(el);
    }
    for (const child of children) {
        el.appendChild(child);
    }
    return el;
}

function dice2text({ dice, bonus}) {
    let text = "";

    for (const {amount, sides, modifiers} of dice) {
        if (text !== "") {
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

export function renderUserInput(domNode, userInput) {
    for (const [name, value] of Object.entries(userInput)) {
        domNode.appendChild(dom("label", null, el => el.innerText = name,
            dom("input", null, el => {
                el.value = value.number ?? dice2text(value.dice);
                el.type = value.number ? "number" : "text";
            })));
    }
}

export function renderDefinitions(domNode, definitions) {
    function stringify(selectorOrLimitor) {
        const args = selectorOrLimitor.arguments;
        return selectorOrLimitor.identifier + (args.length > 0 ? `(${args.join(",")})` : "");
    }

    for (const {name, selector, limiters} of definitions) {
        domNode.appendChild(
            dom("label", null, el => el.innerText = name,
                dom("span", ["badge"], el => el.innerText = stringify(selector)),
                ...limiters.map(limiter => dom("span", ["badge", "warning"], el => el.innerText = stringify(limiter))),
                dom("input", null, el => {
                    el.type = "text";
                    el.disabled = true;
                }),
            )
        );
    }
}

export function renderModifiers(domNode, modifiers) {
    function getModifierValue(modifier) {
        if (modifier.value.script) {
            return modifier.value.script.script;
        } else if (modifier.value.staticValue.dice) {
            return dice2text(modifier.value.staticValue.dice);
        } else {
            return modifier.value.staticValue.number;
        }
    }

    for (const modifier of modifiers) {
        domNode.appendChild(
            dom("label", null, el => el.innerText = modifier.property,
                dom("input", null, el => {
                    el.value = getModifierValue(modifier);
                    el.type = "text";
                    el.disabled = true;
                }),
            )
        );
    }
}

export function renderFeatures(domNode, features) {
    for (const feature of features) {
        domNode.appendChild(
            dom("div", ["card-section", "features"], null,
                dom("h5", null, el => el.innerText = feature.name),
                dom("p", null, el => el.innerText = feature.description),
                dom("div", ["definitions"], el => renderDefinitions(el, feature.definitions)),
                dom("div", ["modifiers"], el => renderModifiers(el, feature.modifiers)),
            )
        );
    }
}

export function renderFeatureSets(domNode, featureSets) {
    for (const [active, featureSet] of featureSets) {
        const inputId = `feature-activated-${featureSet.name}`
        const inputName = `Feature ${featureSet.name}`
        domNode.appendChild(
            dom("div", ["card"], null,
                dom("div", ["card-header"], null,
                    dom("h4", null, null,
                        dom("fieldset", ["form-group"], null,
                            dom("label", ["paper-switch-2"], null,
                                dom("input", null, el => {
                                    el.id = inputId;
                                    el.name = inputName;
                                    el.checked = active;
                                    el.type = "checkbox";
                                }),
                                dom("span", ["paper-switch-slider", "round"])
                            ),
                            // yes, paper.css actually requires two labels for this kind of checkbox
                            dom("label", ["paper-switch-2-label"], el => {
                                el.htmlFor = inputId;
                                el.innerText = featureSet.name;
                            })
                        )
                    )
                ),
                dom("div", ["card-body"], el => renderFeatures(el, featureSet.features),
                    dom("p", ["card-text"], el => el.innerText = featureSet.description),
                )
            )
        );
    }
}

export function render(domNodes, obj) {
    renderUserInput(domNodes.userInputDomNode, obj.userInput)
    renderFeatureSets(domNodes.featureSetsDomNode, obj.featureSets)
}
