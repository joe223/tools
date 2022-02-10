# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > class > declare`

### `ast`

```javascript
JSRoot {
	body: [
		JSClassDeclaration {
			declare: true
			id: JSBindingIdentifier {
				name: "C"
				loc: SourceLocation typescript/class/declare/input.ts 1:14-1:15 (C)
			}
			meta: JSClassHead {
				body: [
					TSIndexSignature {
						readonly: false
						key: JSBindingIdentifier {
							name: "x"
							meta: JSPatternMeta {
								typeAnnotation: TSStringKeywordTypeAnnotation {
									loc: SourceLocation typescript/class/declare/input.ts 2:8-2:14
								}
								loc: SourceLocation typescript/class/declare/input.ts 2:5-2:14
							}
							loc: SourceLocation typescript/class/declare/input.ts 2:5-2:14
						}
						typeAnnotation: TSAnyKeywordTypeAnnotation {
							loc: SourceLocation typescript/class/declare/input.ts 2:17-2:20
						}
						loc: SourceLocation typescript/class/declare/input.ts 2:4-2:21
					}
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "x"
								loc: SourceLocation typescript/class/declare/input.ts 3:4-3:5 (x)
							}
							loc: SourceLocation typescript/class/declare/input.ts 3:4-3:5
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: false
							loc: SourceLocation typescript/class/declare/input.ts 3:4-3:5
							start: Position 3:4
						}
						loc: SourceLocation typescript/class/declare/input.ts 3:4-3:6
					}
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "x"
								loc: SourceLocation typescript/class/declare/input.ts 4:4-4:5 (x)
							}
							loc: SourceLocation typescript/class/declare/input.ts 4:4-4:5
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: false
							loc: SourceLocation typescript/class/declare/input.ts 4:4-4:5
							start: Position 4:4
						}
						typeAnnotation: TSNumberKeywordTypeAnnotation {
							loc: SourceLocation typescript/class/declare/input.ts 4:7-4:13
						}
						loc: SourceLocation typescript/class/declare/input.ts 4:4-4:14
					}
					TSDeclareMethod {
						kind: "method"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "f"
								loc: SourceLocation typescript/class/declare/input.ts 5:4-5:5 (f)
							}
							loc: SourceLocation typescript/class/declare/input.ts 5:4-5:5
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: false
							loc: SourceLocation typescript/class/declare/input.ts 5:4-5:5
							start: Position 5:4
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation typescript/class/declare/input.ts 5:5-5:7
						}
						loc: SourceLocation typescript/class/declare/input.ts 5:4-5:8
					}
					TSDeclareMethod {
						kind: "method"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "f"
								loc: SourceLocation typescript/class/declare/input.ts 6:4-6:5 (f)
							}
							loc: SourceLocation typescript/class/declare/input.ts 6:4-6:5
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: false
							loc: SourceLocation typescript/class/declare/input.ts 6:4-6:5
							start: Position 6:4
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							returnType: TSVoidKeywordTypeAnnotation {
								loc: SourceLocation typescript/class/declare/input.ts 6:9-6:13
							}
							loc: SourceLocation typescript/class/declare/input.ts 6:5-6:13
						}
						loc: SourceLocation typescript/class/declare/input.ts 6:4-6:14
					}
				]
				loc: SourceLocation typescript/class/declare/input.ts 1:0-7:1
			}
			loc: SourceLocation typescript/class/declare/input.ts 1:0-7:1
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/class/declare/input.ts>
	loc: SourceLocation typescript/class/declare/input.ts 1:0-8:0
}
```

### `diagnostics`

```

```