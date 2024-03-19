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

const transformer = (v) => {
	let magic = Math.max(0, Number(v.levels.magic))
	let martial = Math.max(0, Number(v.levels.martial))
	let total = magic + martial
	if (total > 10) {
		total = 10
		if (magic > 10) {
			magic = 10
		}
		martial = 10 - magic
	}
	v = structuredClone(v)
	v.levels.magic = magic
	v.levels.martial = martial
	v.levels.total = total

	const savedCopy = structuredClone(v)
	delete savedCopy.levels.total
	localStorage.setItem('character', JSON.stringify(savedCopy))

	return v
}
const source = new Source(transformer(value), transformer)

source.pick('details').pick('name').bindToDom(domName)
const levels = source.pick('levels')
levels.pick('magic').bindToDom(domMagicLevel)
levels.pick('martial').bindToDom(domMartialLevel)
levels.pick('total').bindToDom(domTotalLevel)

source.bindToDom(domSummary, null, {
	serialize: v => JSON.stringify(v),
	deserialize: v => JSON.parse(v),
})
