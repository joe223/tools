# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > import > not-top-level`

### `ast`

```javascript
JSRoot {
	body: [
		TSModuleDeclaration {
			declare: true
			id: JSStringLiteral {
				value: "m"
				loc: SourceLocation typescript/import/not-top-level/input.ts 1:15-1:18
			}
			body: TSModuleBlock {
				body: [
					JSImportDeclaration {
						namedSpecifiers: []
						namespaceSpecifier: JSImportNamespaceSpecifier {
							local: JSImportSpecifierLocal {
								name: JSBindingIdentifier {
									name: "a"
									loc: SourceLocation typescript/import/not-top-level/input.ts 2:16-2:17 (a)
								}
								loc: SourceLocation typescript/import/not-top-level/input.ts 2:16-2:17
							}
							loc: SourceLocation typescript/import/not-top-level/input.ts 2:4-2:17
						}
						source: JSStringLiteral {
							value: "a"
							loc: SourceLocation typescript/import/not-top-level/input.ts 2:23-2:26
						}
						loc: SourceLocation typescript/import/not-top-level/input.ts 2:4-2:27
					}
				]
				loc: SourceLocation typescript/import/not-top-level/input.ts 1:19-3:1
			}
			loc: SourceLocation typescript/import/not-top-level/input.ts 1:0-3:1
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/import/not-top-level/input.ts>
	loc: SourceLocation typescript/import/not-top-level/input.ts 1:0-4:0
}
```

### `diagnostics`

```

```