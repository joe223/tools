# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > type-alias > export`

### `ast`

```javascript
JSRoot {
	body: [
		JSExportLocalDeclaration {
			exportKind: "type"
			trailingComments: ["0"]
			declaration: TSTypeAlias {
				id: JSBindingIdentifier {
					name: "T"
					loc: SourceLocation typescript/type-alias/export/input.ts 1:12-1:13 (T)
				}
				right: TSNumberKeywordTypeAnnotation {
					loc: SourceLocation typescript/type-alias/export/input.ts 1:16-1:22
				}
				loc: SourceLocation typescript/type-alias/export/input.ts 1:7-1:23
			}
			loc: SourceLocation typescript/type-alias/export/input.ts 1:0-1:23
		}
	]
	comments: [
		CommentLine {
			id: "0"
			value: " `export default type` is not valid."
			loc: SourceLocation typescript/type-alias/export/input.ts 2:0-2:38
		}
	]
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/type-alias/export/input.ts>
	loc: SourceLocation typescript/type-alias/export/input.ts 1:0-3:0
}
```

### `diagnostics`

```

```