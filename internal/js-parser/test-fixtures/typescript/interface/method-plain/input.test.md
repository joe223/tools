# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > interface > method-plain`

### `ast`

```javascript
JSRoot {
	body: [
		TSInterfaceDeclaration {
			id: JSBindingIdentifier {
				name: "I"
				loc: SourceLocation typescript/interface/method-plain/input.ts 1:10-1:11 (I)
			}
			body: TSInterfaceBody {
				body: [
					TSMethodSignature {
						optional: false
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "m"
								loc: SourceLocation typescript/interface/method-plain/input.ts 2:4-2:5 (m)
							}
							loc: SourceLocation typescript/interface/method-plain/input.ts 2:4-2:5
						}
						meta: TSSignatureDeclarationMeta {
							parameters: []
							loc: SourceLocation typescript/interface/method-plain/input.ts 2:5-2:7
						}
						loc: SourceLocation typescript/interface/method-plain/input.ts 2:4-2:8
					}
					TSMethodSignature {
						optional: false
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "m"
								loc: SourceLocation typescript/interface/method-plain/input.ts 3:4-3:5 (m)
							}
							loc: SourceLocation typescript/interface/method-plain/input.ts 3:4-3:5
						}
						meta: TSSignatureDeclarationMeta {
							parameters: [
								JSBindingIdentifier {
									name: "x"
									meta: JSPatternMeta {
										optional: true
										typeAnnotation: TSNumberKeywordTypeAnnotation {
											loc: SourceLocation typescript/interface/method-plain/input.ts 3:10-3:16
										}
										loc: SourceLocation typescript/interface/method-plain/input.ts 3:6-3:16
									}
									loc: SourceLocation typescript/interface/method-plain/input.ts 3:6-3:7 (x)
								}
							]
							rest: JSBindingIdentifier {
								name: "y"
								meta: JSPatternMeta {
									typeAnnotation: TSArrayType {
										elementType: TSNumberKeywordTypeAnnotation {
											loc: SourceLocation typescript/interface/method-plain/input.ts 3:24-3:30
										}
										loc: SourceLocation typescript/interface/method-plain/input.ts 3:24-3:32
									}
									loc: SourceLocation typescript/interface/method-plain/input.ts 3:21-3:32
								}
								loc: SourceLocation typescript/interface/method-plain/input.ts 3:21-3:22 (y)
							}
							loc: SourceLocation typescript/interface/method-plain/input.ts 3:5-3:39
						}
						returnType: TSVoidKeywordTypeAnnotation {
							loc: SourceLocation typescript/interface/method-plain/input.ts 3:35-3:39
						}
						loc: SourceLocation typescript/interface/method-plain/input.ts 3:4-3:40
					}
				]
				loc: SourceLocation typescript/interface/method-plain/input.ts 1:12-4:1
			}
			loc: SourceLocation typescript/interface/method-plain/input.ts 1:0-4:1
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/interface/method-plain/input.ts>
	loc: SourceLocation typescript/interface/method-plain/input.ts 1:0-5:0
}
```

### `diagnostics`

```

```