# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > const > reserved-word`

### `ast`

```javascript
JSRoot {
	body: [
		JSVariableDeclarationStatement {
			declaration: JSVariableDeclaration {
				kind: "const"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingIdentifier {
							name: "b"
							meta: JSPatternMeta {
								typeAnnotation: TSTypeReference {
									typeName: JSReferenceIdentifier {
										name: "INVALID_PLACEHOLDER"
										loc: SourceLocation typescript/const/reserved-word/input.ts 1:14-1:14
									}
									loc: SourceLocation typescript/const/reserved-word/input.ts 1:14-1:14
								}
								loc: SourceLocation typescript/const/reserved-word/input.ts 1:6-1:14
							}
							loc: SourceLocation typescript/const/reserved-word/input.ts 1:6-1:14
						}
						loc: SourceLocation typescript/const/reserved-word/input.ts 1:6-1:14
					}
				]
				loc: SourceLocation typescript/const/reserved-word/input.ts 1:0-1:15
			}
			loc: SourceLocation typescript/const/reserved-word/input.ts 1:0-1:15
		}
	]
	comments: []
	corrupt: true
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: []
				category: ["parse"]
				categoryValue: "js"
				message: RAW_MARKUP {value: "Unknown TS non array type start"}
			}
			location: {
				language: "js"
				path: UIDPath<typescript/const/reserved-word/input.ts>
				end: Position 1:8
				start: Position 1:9
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/const/reserved-word/input.ts>
	loc: SourceLocation typescript/const/reserved-word/input.ts 1:0-1:15
}
```

### `diagnostics`

```

 typescript/const/reserved-word/input.ts:1:9 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unknown TS non array type start

    const b: const;
             ^


```