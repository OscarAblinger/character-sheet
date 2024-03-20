class AbstractPipe {
	constructor() {
		this.observers = []
	}
	getValue() { throw 'Not implemented' }
	updateValue(val) { throw 'Not implemented' }

	subscribe(observer) { this.observers.push(observer) }
	unsubscribe(observer) { this.observers = this.observers.filter(o => o !== observer) }
	notifySubscribers() { this.observers.forEach(o => o.notify(this)) }
	notify(caller) { this.notifySubscribers(this) }

	pick(prop) { return new PickPipe(this, prop) }
	bindToDom(element, elementProp, serializer) { return new DomBinder({pipe: this, element, elementProp, serializer}) }
}

class Source extends AbstractPipe {
	constructor(initialValue, onChangeTransformer) {
		super()
		if (onChangeTransformer == null)
                        onChangeTransformer = (orig, changes) => Object.assign({}, orig, changes)

		this.value = initialValue
		this.onChangeTransformer = onChangeTransformer
		this.notifySubscribers()
	}
	getValue() { return this.value }
	updateValue(val) {
		if (val === this.value)
			return

		this.value = this.onChangeTransformer(this.value, val)
		this.notifySubscribers()
	}
}

class PickPipe extends AbstractPipe {
        constructor(parent, prop) {
                super()
                this.parent = parent
		this.prop = prop
		this.parent.subscribe(this)
        }
	getValue() {
		return this.parent.getValue()?.[this.prop]
	}
	updateValue(val) {
		this.parent.updateValue({ [this.prop]: val })
	}
}

const DEFAULT_SERIALIZER = {
	serialize: v => v,
	deserialize: v => v,
}

function tryDeduceValuePropForElement(element) {
	return element.value != null ? 'value' : 'textContent' // note that textContent ignores <br>
}

class DomBinder {
	constructor({pipe, element, elementProp, serializer}) {
		if (serializer == null) 
			serializer = DEFAULT_SERIALIZER
		if (elementProp == null)
                        elementProp = tryDeduceValuePropForElement(element)
		this.pipe = pipe
		this.element = element
		this.elementProp = elementProp
		this.serializer = serializer

		this.pipe.subscribe(this)

		const onDomRefresh = () => {
			const value = this.element[this.elementProp]
			this.pipe.updateValue(this.serializer.deserialize(value))
		}
		this.element.addEventListener('change', () => onDomRefresh())
		this.#refreshDom()
	}
	notify() { this.#refreshDom() }

	#refreshDom() {
		this.element[this.elementProp] = this.serializer.serialize(this.pipe.getValue())
	}
}
