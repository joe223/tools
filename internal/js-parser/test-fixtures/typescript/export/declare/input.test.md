# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > export > declare`

### `ast`

```javascript
JSRoot {
	body: [
		JSExportLocalDeclaration {
			exportKind: "value"
			declaration: JSVariableDeclarationStatement {
				declare: true
				declaration: JSVariableDeclaration {
					kind: "const"
					declarations: [
						JSVariableDeclarator {
							id: JSBindingIdentifier {
								name: "x"
								meta: JSPatternMeta {
									typeAnnotation: TSNumberKeywordTypeAnnotation {
										loc: SourceLocation typescript/export/declare/input.ts 1:24-1:30
									}
									loc: SourceLocation typescript/export/declare/input.ts 1:21-1:30
								}
								loc: SourceLocation typescript/export/declare/input.ts 1:21-1:30
							}
							loc: SourceLocation typescript/export/declare/input.ts 1:21-1:30
						}
					]
					loc: SourceLocation typescript/export/declare/input.ts 1:7-1:31
				}
				loc: SourceLocation typescript/export/declare/input.ts 1:7-1:31
			}
			loc: SourceLocation typescript/export/declare/input.ts 1:0-1:31
		}
		JSExportLocalDeclaration {
			exportKind: "value"
			declaration: JSFunctionDeclaration {
				declare: true
				id: JSBindingIdentifier {
					name: "function"
					loc: SourceLocation typescript/export/declare/input.ts 2:15-2:23 (function)
				}
				body: JSBlockStatement {
					body: [
						JSExpressionStatement {
							expression: JSReferenceIdentifier {
								name: "INVALID_PLACEHOLDER"
								loc: SourceLocation typescript/export/declare/input.ts 2:26-2:26
							}
							loc: SourceLocation typescript/export/declare/input.ts 2:25-2:27
						}
						JSExpressionStatement {
							expression: JSReferenceIdentifier {
								name: "INVALID_PLACEHOLDER"
								loc: SourceLocation typescript/export/declare/input.ts 2:27-2:28
							}
							loc: SourceLocation typescript/export/declare/input.ts 2:27-2:28
						}
						JSExpressionStatement {
							expression: JSUnaryExpression {
								operator: "void"
								prefix: true
								argument: JSReferenceIdentifier {
									name: "INVALID_PLACEHOLDER"
									loc: SourceLocation typescript/export/declare/input.ts 2:33-2:34
								}
								loc: SourceLocation typescript/export/declare/input.ts 2:29-2:34
							}
							loc: SourceLocation typescript/export/declare/input.ts 2:29-2:34
						}
						JSExportLocalDeclaration {
							exportKind: "value"
							declaration: JSClassDeclaration {
								declare: true
								id: JSBindingIdentifier {
									name: "C"
									loc: SourceLocation typescript/export/declare/input.ts 3:21-3:22 (C)
								}
								meta: JSClassHead {
									body: []
									loc: SourceLocation typescript/export/declare/input.ts 3:7-3:25
								}
								loc: SourceLocation typescript/export/declare/input.ts 3:7-3:25
							}
							loc: SourceLocation typescript/export/declare/input.ts 3:0-3:25
						}
						JSExportLocalDeclaration {
							exportKind: "value"
							declaration: TSInterfaceDeclaration {
								declare: true
								id: JSBindingIdentifier {
									name: "I"
									loc: SourceLocation typescript/export/declare/input.ts 4:25-4:26 (I)
								}
								body: TSInterfaceBody {
									body: []
									loc: SourceLocation typescript/export/declare/input.ts 4:27-4:29
								}
								loc: SourceLocation typescript/export/declare/input.ts 4:7-4:29
							}
							loc: SourceLocation typescript/export/declare/input.ts 4:0-4:29
						}
						JSExportLocalDeclaration {
							exportKind: "value"
							declaration: TSTypeAlias {
								declare: true
								id: JSBindingIdentifier {
									name: "T"
									loc: SourceLocation typescript/export/declare/input.ts 5:20-5:21 (T)
								}
								right: TSNumberKeywordTypeAnnotation {
									loc: SourceLocation typescript/export/declare/input.ts 5:24-5:30
								}
								loc: SourceLocation typescript/export/declare/input.ts 5:7-5:31
							}
							loc: SourceLocation typescript/export/declare/input.ts 5:0-5:31
						}
						JSExportLocalDeclaration {
							exportKind: "value"
							declaration: TSModuleDeclaration {
								declare: true
								id: JSBindingIdentifier {
									name: "M"
									loc: SourceLocation typescript/export/declare/input.ts 6:22-6:23 (M)
								}
								body: TSModuleBlock {
									body: []
									loc: SourceLocation typescript/export/declare/input.ts 6:24-6:26
								}
								loc: SourceLocation typescript/export/declare/input.ts 6:7-6:26
							}
							loc: SourceLocation typescript/export/declare/input.ts 6:0-6:26
						}
						JSExportLocalDeclaration {
							exportKind: "value"
							declaration: TSModuleDeclaration {
								declare: true
								id: JSBindingIdentifier {
									name: "N"
									loc: SourceLocation typescript/export/declare/input.ts 7:25-7:26 (N)
								}
								body: TSModuleBlock {
									body: []
									loc: SourceLocation typescript/export/declare/input.ts 7:27-7:29
								}
								loc: SourceLocation typescript/export/declare/input.ts 7:7-7:29
							}
							loc: SourceLocation typescript/export/declare/input.ts 7:0-7:29
						}
						JSExportLocalDeclaration {
							exportKind: "value"
							declaration: TSEnumDeclaration {
								const: false
								declare: true
								members: []
								id: JSBindingIdentifier {
									name: "foo"
									loc: SourceLocation typescript/export/declare/input.ts 8:20-8:23 (foo)
								}
								loc: SourceLocation typescript/export/declare/input.ts 8:7-8:26
							}
							loc: SourceLocation typescript/export/declare/input.ts 8:0-8:26
						}
					]
					directives: []
					loc: SourceLocation typescript/export/declare/input.ts 2:25-8:26
				}
				head: JSFunctionHead {
					async: false
					generator: false
					hasHoistedVars: false
					params: [
						JSBindingIdentifier {
							name: "f"
							meta: JSPatternMeta {
								loc: SourceLocation typescript/export/declare/input.ts 2:24-2:25
							}
							loc: SourceLocation typescript/export/declare/input.ts 2:24-2:25 (f)
						}
					]
					loc: SourceLocation typescript/export/declare/input.ts 2:24-2:25
				}
				loc: SourceLocation typescript/export/declare/input.ts 2:7-8:26
			}
			loc: SourceLocation typescript/export/declare/input.ts 2:0-8:26
		}
	]
	comments: []
	corrupt: true
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: [
					log {
						category: "info"
						text: [
							RAW_MARKUP {value: "Expected the opening "}
							"function params"
							RAW_MARKUP {value: " character <emphasis>"}
							"("
							RAW_MARKUP {value: "</emphasis>"}
						]
					}
				]
				category: ["parse"]
				categoryValue: "js"
				message: [RAW_MARKUP {value: "Unexpected character <emphasis>"}, "f", RAW_MARKUP {value: "</emphasis>"}]
			}
			location: {
				language: "js"
				path: UIDPath<typescript/export/declare/input.ts>
				end: Position 2:23
				start: Position 2:24
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/export/declare/input.ts>
	loc: SourceLocation typescript/export/declare/input.ts 1:0-9:0
}
```

### `diagnostics`

```

 typescript/export/declare/input.ts:2:24 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unexpected character f

    1 │ export declare const x: number;
  > 2 │ export declare function f(): void;
      │                         ^
    3 │ export declare class C {}
    4 │ export declare interface I {}

  ℹ Expected the opening function params character (


```