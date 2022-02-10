# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > types > read-only-4`

### `ast`

```javascript
JSRoot {
	body: [
		TSTypeAlias {
			trailingComments: ["0"]
			id: JSBindingIdentifier {
				name: "T33"
				loc: SourceLocation typescript/types/read-only-4/input.ts 1:5-1:8 (T33)
			}
			right: TSTypeOperator {
				operator: "readonly"
				typeAnnotation: TSTypeReference {
					typeName: JSReferenceIdentifier {
						name: "Array"
						loc: SourceLocation typescript/types/read-only-4/input.ts 1:20-1:25 (Array)
					}
					typeParameters: TSTypeParameterInstantiation {
						params: [
							TSStringKeywordTypeAnnotation {
								loc: SourceLocation typescript/types/read-only-4/input.ts 1:26-1:32
							}
						]
						loc: SourceLocation typescript/types/read-only-4/input.ts 1:25-1:33
					}
					loc: SourceLocation typescript/types/read-only-4/input.ts 1:20-1:33
				}
				loc: SourceLocation typescript/types/read-only-4/input.ts 1:11-1:33
			}
			loc: SourceLocation typescript/types/read-only-4/input.ts 1:0-1:34
		}
	]
	comments: [
		CommentLine {
			id: "0"
			value: " Error"
			loc: SourceLocation typescript/types/read-only-4/input.ts 1:36-1:44
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
				path: UIDPath<typescript/types/read-only-4/input.ts>
				end: Position 1:33
				start: Position 1:20
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/types/read-only-4/input.ts>
	loc: SourceLocation typescript/types/read-only-4/input.ts 1:0-2:0
}
```

### `diagnostics`

```

 typescript/types/read-only-4/input.ts:1:20 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ 'readonly' type modifier is only permitted on array and tuple literal types.

    type T33 = readonly Array<string>;  // Error
                        ^^^^^^^^^^^^^


```