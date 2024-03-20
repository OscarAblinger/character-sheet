const lsValue = localStorage.getItem('character');
const DEFAULT_VALUE = {
	details: {
		name: "Blizzard the Wizzard",
	},
	levels: {
		magic: 3,
		martial: 2,
	},
}

const value = lsValue ? JSON.parse(lsValue) : DEFAULT_VALUE;

const domName = document.getElementById('charname')
const domTotalLevel = document.getElementById('level')
const domMagicLevel = document.getElementById('magiclevel')
const domMartialLevel = document.getElementById('martiallevel')
const domSummary = document.getElementById('summary')

const transformer = (orig, update) => {
	let magic = Math.max(0, Number(update?.levels?.magic || orig?.levels?.magic))
	let martial = Math.max(0, Number(update?.levels?.martial || orig?.levels?.martial))
	magic = Math.min(magic, 10)
	martial = Math.min(martial, 10)

	let total = magic + martial
	if (total > 10) {
		total = 10
		if (update?.levels?.martial) {
			// martial was updated -> we reduce magic instead
			magic = 10 - martial
                } else {
			martial = 10 - magic
		}
	}

	const newVal = structuredClone(orig)
	newVal.levels.magic = magic
	newVal.levels.martial = martial
	newVal.levels.total = total

	const savedCopy = structuredClone(newVal)
	delete savedCopy.levels.total
	localStorage.setItem('character', JSON.stringify(savedCopy))

	return newVal
}
const source = new Source(transformer(value, {}), transformer)

source.pick('details').pick('name').bindToDom(domName)
const levels = source.pick('levels')
levels.pick('magic').bindToDom(domMagicLevel)
levels.pick('martial').bindToDom(domMartialLevel)
levels.pick('total').bindToDom(domTotalLevel)

source.bindToDom(domSummary, null, {
	serialize: v => JSON.stringify(v),
	deserialize: v => JSON.parse(v),
})
