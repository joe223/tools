# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > assert-predicate > function-declaration`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "asserts1"
				loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 1:9-1:17 (asserts1)
			}
			body: JSBlockStatement {
				body: []
				directives: []
				loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 1:60-1:62
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: false
				params: [
					JSBindingIdentifier {
						name: "value"
						meta: JSPatternMeta {
							typeAnnotation: TSUnknownKeywordTypeAnnotation {
								loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 1:26-1:33
							}
							loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 1:19-1:33
						}
						loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 1:19-1:24 (value)
					}
				]
				returnType: TSTypePredicate {
					asserts: true
					parameterName: JSIdentifier {
						name: "value"
						loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 1:44-1:49 (value)
					}
					typeAnnotation: TSStringKeywordTypeAnnotation {
						loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 1:53-1:59
					}
					loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 1:44-1:59
				}
				loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 1:18-1:59
			}
			loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 1:0-1:62
		}
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "asserts2"
				loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 2:9-2:17 (asserts2)
			}
			body: JSBlockStatement {
				body: []
				directives: []
				loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 2:50-2:52
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: false
				params: [
					JSBindingIdentifier {
						name: "value"
						meta: JSPatternMeta {
							typeAnnotation: TSUnknownKeywordTypeAnnotation {
								loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 2:26-2:33
							}
							loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 2:19-2:33
						}
						loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 2:19-2:24 (value)
					}
				]
				returnType: TSTypePredicate {
					asserts: true
					parameterName: JSIdentifier {
						name: "value"
						loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 2:44-2:49 (value)
					}
					loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 2:34-2:49
				}
				loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 2:18-2:49
			}
			loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 2:0-2:52
		}
		TSDeclareFunction {
			id: JSBindingIdentifier {
				name: "asserts3"
				loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 3:9-3:17 (asserts3)
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: false
				params: [
					JSBindingIdentifier {
						name: "value"
						meta: JSPatternMeta {
							typeAnnotation: TSUnknownKeywordTypeAnnotation {
								loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 3:26-3:33
							}
							loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 3:19-3:33
						}
						loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 3:19-3:24 (value)
					}
				]
				returnType: TSTypePredicate {
					asserts: true
					parameterName: JSIdentifier {
						name: ""
						loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 3:44-3:45 ()
					}
					loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 3:34-3:45
				}
				loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 3:18-3:45
			}
			loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 3:0-3:45
		}
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "INVALID_PLACEHOLDER"
				loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 3:45-3:46
			}
			loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 3:45-3:46
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
				message: RAW_MARKUP {value: "Expected an identifier"}
			}
			location: {
				language: "js"
				path: UIDPath<typescript/assert-predicate/function-declaration/input.ts>
				end: Position 3:43
				start: Position 3:44
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/assert-predicate/function-declaration/input.ts>
	loc: SourceLocation typescript/assert-predicate/function-declaration/input.ts 1:0-4:0
}
```

### `diagnostics`

```

 typescript/assert-predicate/function-declaration/input.ts:3:44 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Expected an identifier

    1 │ function asserts1 (value: unknown): asserts value is string {}·
    2 │ function asserts2 (value: unknown): asserts value {}
  > 3 │ function asserts3 (value: unknown): asserts {}
      │                                             ^


```