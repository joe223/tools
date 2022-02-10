# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > types > tuple`

### `ast`

```javascript
JSRoot {
	body: [
		JSVariableDeclarationStatement {
			declaration: JSVariableDeclaration {
				kind: "let"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingIdentifier {
							name: "x"
							meta: JSPatternMeta {
								typeAnnotation: TSTupleType {
									elementTypes: [
										TSTupleElement {
											optional: false
											typeAnnotation: TSNumberKeywordTypeAnnotation {
												loc: SourceLocation typescript/types/tuple/input.ts 1:8-1:14
											}
											loc: SourceLocation typescript/types/tuple/input.ts 1:8-1:14
										}
										TSTupleElement {
											optional: false
											typeAnnotation: TSNumberKeywordTypeAnnotation {
												loc: SourceLocation typescript/types/tuple/input.ts 1:16-1:22
											}
											loc: SourceLocation typescript/types/tuple/input.ts 1:16-1:22
										}
										TSTupleElement {
											optional: false
											typeAnnotation: TSNumberKeywordTypeAnnotation {
												loc: SourceLocation typescript/types/tuple/input.ts 1:24-1:30
											}
											loc: SourceLocation typescript/types/tuple/input.ts 1:24-1:30
										}
									]
									loc: SourceLocation typescript/types/tuple/input.ts 1:7-1:31
								}
								loc: SourceLocation typescript/types/tuple/input.ts 1:4-1:31
							}
							loc: SourceLocation typescript/types/tuple/input.ts 1:4-1:31
						}
						loc: SourceLocation typescript/types/tuple/input.ts 1:4-1:31
					}
				]
				loc: SourceLocation typescript/types/tuple/input.ts 1:0-1:32
			}
			loc: SourceLocation typescript/types/tuple/input.ts 1:0-1:32
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/types/tuple/input.ts>
	loc: SourceLocation typescript/types/tuple/input.ts 1:0-2:0
}
```

### `diagnostics`

```

```