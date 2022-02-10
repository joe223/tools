# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > function > overloads`

### `ast`

```javascript
JSRoot {
	body: [
		JSExportLocalDeclaration {
			exportKind: "value"
			declaration: TSDeclareFunction {
				id: JSBindingIdentifier {
					name: "f"
					loc: SourceLocation typescript/function/overloads/input.ts 1:16-1:17 (f)
				}
				head: JSFunctionHead {
					async: false
					generator: false
					hasHoistedVars: false
					params: [
						JSBindingIdentifier {
							name: "x"
							meta: JSPatternMeta {
								typeAnnotation: TSNumberKeywordTypeAnnotation {
									loc: SourceLocation typescript/function/overloads/input.ts 1:21-1:27
								}
								loc: SourceLocation typescript/function/overloads/input.ts 1:18-1:27
							}
							loc: SourceLocation typescript/function/overloads/input.ts 1:18-1:19 (x)
						}
					]
					returnType: TSNumberKeywordTypeAnnotation {
						loc: SourceLocation typescript/function/overloads/input.ts 1:30-1:36
					}
					loc: SourceLocation typescript/function/overloads/input.ts 1:17-1:36
				}
				loc: SourceLocation typescript/function/overloads/input.ts 1:7-1:37
			}
			loc: SourceLocation typescript/function/overloads/input.ts 1:0-1:37
		}
		JSExportLocalDeclaration {
			exportKind: "value"
			declaration: TSDeclareFunction {
				id: JSBindingIdentifier {
					name: "f"
					loc: SourceLocation typescript/function/overloads/input.ts 2:16-2:17 (f)
				}
				head: JSFunctionHead {
					async: false
					generator: false
					hasHoistedVars: false
					params: [
						JSBindingIdentifier {
							name: "x"
							meta: JSPatternMeta {
								typeAnnotation: TSStringKeywordTypeAnnotation {
									loc: SourceLocation typescript/function/overloads/input.ts 2:21-2:27
								}
								loc: SourceLocation typescript/function/overloads/input.ts 2:18-2:27
							}
							loc: SourceLocation typescript/function/overloads/input.ts 2:18-2:19 (x)
						}
					]
					returnType: TSStringKeywordTypeAnnotation {
						loc: SourceLocation typescript/function/overloads/input.ts 2:30-2:36
					}
					loc: SourceLocation typescript/function/overloads/input.ts 2:17-2:36
				}
				loc: SourceLocation typescript/function/overloads/input.ts 2:7-2:37
			}
			loc: SourceLocation typescript/function/overloads/input.ts 2:0-2:37
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/function/overloads/input.ts>
	loc: SourceLocation typescript/function/overloads/input.ts 1:0-3:0
}
```

### `diagnostics`

```

```