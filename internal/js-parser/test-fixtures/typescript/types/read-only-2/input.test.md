# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > types > read-only-2`

### `ast`

```javascript
JSRoot {
	body: [
		TSTypeAlias {
			trailingComments: ["0"]
			id: JSBindingIdentifier {
				name: "T31"
				loc: SourceLocation typescript/types/read-only-2/input.ts 1:5-1:8 (T31)
			}
			right: TSTypeOperator {
				operator: "readonly"
				typeAnnotation: TSTypeReference {
					typeName: JSReferenceIdentifier {
						name: "T"
						loc: SourceLocation typescript/types/read-only-2/input.ts 1:20-1:21 (T)
					}
					loc: SourceLocation typescript/types/read-only-2/input.ts 1:20-1:21
				}
				loc: SourceLocation typescript/types/read-only-2/input.ts 1:11-1:21
			}
			loc: SourceLocation typescript/types/read-only-2/input.ts 1:0-1:22
		}
	]
	comments: [
		CommentLine {
			id: "0"
			value: " Error"
			loc: SourceLocation typescript/types/read-only-2/input.ts 1:24-1:32
		}
	]
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: []
				category: ["parse"]
				categoryValue: "js"
				message: RAW_MARKUP {value: "'readonly' type modifier is only permitted on array and tuple literal types."}
			}
			location: {
				language: "js"
				path: UIDPath<typescript/types/read-only-2/input.ts>
				end: Position 1:21
				start: Position 1:20
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/types/read-only-2/input.ts>
	loc: SourceLocation typescript/types/read-only-2/input.ts 1:0-1:32
}
```

### `diagnostics`

```

 typescript/types/read-only-2/input.ts:1:20 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ 'readonly' type modifier is only permitted on array and tuple literal types.

    type T31 = readonly T;  // Error
                        ^


```