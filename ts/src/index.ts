import fs from 'node:fs/promises';

type DataResult = {
	id: string,
	isActive: boolean,
	registered: string,
	tags: string[]
}

async function main() {
	const file = await fs.readFile('../giant-json.json');

	if (!file) 
		console.log("File should open");

	const dataResult: Array<DataResult> = JSON.parse(file.toString());

	const tagUsers: Record<string, string[]> = {};

	for (let user of dataResult) {
		for (let tag of user.tags) {
			if (!tagUsers[tag]) {
				tagUsers[tag] = [user.id];
			} else {
				tagUsers[tag].push(user.id);
			}
		}
	}

	console.log(`total users -> ${dataResult.length}`);
	console.log(`total tags -> ${Object.keys(tagUsers).length}`);

	console.log('total users by tags');
	for (let tag of Object.keys(tagUsers)) {
		console.log(`tag ${tag} -> ${tagUsers[tag].length} users`);
	}
}

main()

