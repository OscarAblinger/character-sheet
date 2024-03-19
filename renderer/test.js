const notYetCreated = Symbol('notYetCreated')

class BidiObservable {
	constructor(pullUpTransformer) {
		this.pullUpTransformer = pullUpTransformer
		this.callbacks = []
		this.lastValue = notYetCreated
	}

	pick(name) {
		const pickBidiObservable = new BidiObservable((d) => this.pullUp({[name]: d}))
		const pushdownCallback = (d) => pickBidiObservable.pushDown(d == null ? undefined : d[name])
		this.callbacks.push(pushdownCallback)
		if (this.lastValue !== notYetCreated) {
                        pushdownCallback(this.lastValue)
                }

                return pickBidiObservable
	}

	subscribe(callback) {
		this.callbacks.push(callback)
		callback(this.lastValue)
		return () => this.callbacks.remove(callback)
	}

	pullUp(value) {
		console.log({pullUp: value})
		this.pullUpTransformer(value)
	}

	pushDown(value) {
		console.log({lastValue: this.lastValue, value})
		if (this.lastValue !== value) {
			this.lastValue = value
			for (const callback of this.callbacks) {
				callback(value)
			}
		}
	}
}

function isObject(o) {
        return o && typeof o === 'object' && !Array.isArray(o)
}

// base is modified in this function
function deepMerge(base, extension) {
	if (!isObject(base)) {
                return extension
        }

	if (isObject(extension)) {
		for (const key in extension) {
			if (isObject(extension[key])) {
				if (!isObject(base[key])) {
                                        Object.assign(base[key], { [key]: {} })
                                }
				deepMerge(base[key], extension[key])
			} else {
				Object.assign(base, { [key]: extension[key] })
			}
		}
	}
	return base
}

const globalVal = {
	name: 'John Doe',
	character: {
		name: 'Blizzard the Wizzard',
		class: 'Wizard',
		level: 2,
	}
}

console.log({globalVal})

const topOb = new BidiObservable()
topOb.pullUpTransformer = (d) => {
	console.log({topObPullUp: d})
	deepMerge(globalVal, d)
	console.log({pushingDown: globalVal})
	topOb.pushDown(globalVal)
}
topOb.pushDown(globalVal)

const nameOb = topOb.pick('name')
nameOb.subscribe(d => console.log('name changed to ', d))
const charNameOb = topOb.pick('character').pick('name')
charNameOb.subscribe(d => console.log('character.name changed to ', d))
const charLevelOb = topOb.pick('character').pick('level')
charLevelOb.subscribe(d => console.log('character.level changed to ', d))

console.log({globalVal})
charLevelOb.pullUp(3)

console.log({globalVal})
