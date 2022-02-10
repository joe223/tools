# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > import > export-import`

### `ast`

```javascript
JSRoot {
	body: [
		TSImportEqualsDeclaration {
			isExport: true
			id: JSBindingIdentifier {
				name: "A"
				loc: SourceLocation typescript/import/export-import/input.ts 1:14-1:15 (A)
			}
			moduleReference: TSQualifiedName {
				left: JSReferenceIdentifier {
					name: "B"
					loc: SourceLocation typescript/import/export-import/input.ts 1:18-1:19 (B)
				}
				right: JSIdentifier {
					name: "C"
					loc: SourceLocation typescript/import/export-import/input.ts 1:20-1:21 (C)
				}
				loc: SourceLocation typescript/import/export-import/input.ts 1:18-1:21
			}
			loc: SourceLocation typescript/import/export-import/input.ts 1:0-1:22
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/import/export-import/input.ts>
	loc: SourceLocation typescript/import/export-import/input.ts 1:0-2:0
}
```

### `diagnostics`

```

```