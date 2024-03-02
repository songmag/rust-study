Assertions (AssertJ Core 3.25.3 API)var pathtoroot = "../../../../";
loadScripts(document, 'script');\<div\>JavaScript is disabled on your browser.\</div\>

[Skip navigation links](#skip-navbar-top)

* [Overview](../../../../index.html)
* [Package](package-summary.html)
* Class
* [Use](class-use/Assertions.html)
* [Tree](package-tree.html)
* [Deprecated](../../../../deprecated-list.html)
* [Index](../../../../index-all.html)

* Summary:

  * Nested
  * [Field](#field-summary)
  * [Constr](#constructor-summary)
  * [Method](#method-summary)

* Detail:

  * Field
  * [Constr](#constructor-detail)
  * [Method](#method-detail)

* Summary: 
* Nested | 
* [Field](#field-summary) | 
* [Constr](#constructor-summary) | 
* [Method](#method-summary)

* Detail: 
* Field | 
* [Constr](#constructor-detail) | 
* [Method](#method-detail)

[SEARCH](../../../../search.html)

Package [org.assertj.core.api](package-summary.html)

Class Assertions
==========

[java.lang.Object](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html)

org.assertj.core.api.Assertions

All Implemented Interfaces:`[InstanceOfAssertFactories](InstanceOfAssertFactories.html)`Direct Known Subclasses:`[BDDAssertions](BDDAssertions.html)`
---

public class Assertionsextends [Object](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html)implements [InstanceOfAssertFactories](InstanceOfAssertFactories.html)

Entry point for assertion methods for different types. Each method in this class is a static factory for a type-specific assertion object.

 For example:

```
 int removed = employees.removeFired();
 assertThat(removed).isZero();

 List<Employee> newEmployees = employees.hired(TODAY);
 assertThat(newEmployees).hasSize(6);
```

 This class only contains all `assertThat` methods, if you have ambiguous method compilation error, use either [`AssertionsForClassTypes`](AssertionsForClassTypes.html) or [`AssertionsForInterfaceTypes`](AssertionsForInterfaceTypes.html) and if you need both, fully qualify you assertThat method.

 Java 8 is picky when choosing the right `assertThat` method if the object under test is generic and bounded, for example if foo is instance of T that extends Exception, java 8 will complain that it can't resolve the proper `assertThat` method (normally `assertThat(Throwable)` as foo might implement an interface like List, if that occurred `assertThat(List)` would also be a possible choice - thus confusing java 8.

 This why [`Assertions`](Assertions.html) have been split in [`AssertionsForClassTypes`](AssertionsForClassTypes.html) and [`AssertionsForInterfaceTypes`](AssertionsForInterfaceTypes.html) (see http://stackoverflow.com/questions/29499847/ambiguous-method-in-java-8-why).

Author:Alex Ruiz, Yvonne Wang, David DIDIER, Ted Young, Joel Costigliola, Matthieu Baechler, Mikhail Mazursky, Nicolas François, Julien Meddah, William Bakker, William Delanoue

* Field Summary
  ----------

  ### Fields inherited from interface org.assertj.core.api.[InstanceOfAssertFactories](InstanceOfAssertFactories.html) ###

  `[ARRAY](InstanceOfAssertFactories.html#ARRAY), [ARRAY_2D](InstanceOfAssertFactories.html#ARRAY_2D), [ATOMIC_BOOLEAN](InstanceOfAssertFactories.html#ATOMIC_BOOLEAN), [ATOMIC_INTEGER](InstanceOfAssertFactories.html#ATOMIC_INTEGER), [ATOMIC_INTEGER_ARRAY](InstanceOfAssertFactories.html#ATOMIC_INTEGER_ARRAY), [ATOMIC_INTEGER_FIELD_UPDATER](InstanceOfAssertFactories.html#ATOMIC_INTEGER_FIELD_UPDATER), [ATOMIC_LONG](InstanceOfAssertFactories.html#ATOMIC_LONG), [ATOMIC_LONG_ARRAY](InstanceOfAssertFactories.html#ATOMIC_LONG_ARRAY), [ATOMIC_LONG_FIELD_UPDATER](InstanceOfAssertFactories.html#ATOMIC_LONG_FIELD_UPDATER), [ATOMIC_MARKABLE_REFERENCE](InstanceOfAssertFactories.html#ATOMIC_MARKABLE_REFERENCE), [ATOMIC_REFERENCE](InstanceOfAssertFactories.html#ATOMIC_REFERENCE), [ATOMIC_REFERENCE_ARRAY](InstanceOfAssertFactories.html#ATOMIC_REFERENCE_ARRAY), [ATOMIC_REFERENCE_FIELD_UPDATER](InstanceOfAssertFactories.html#ATOMIC_REFERENCE_FIELD_UPDATER), [ATOMIC_STAMPED_REFERENCE](InstanceOfAssertFactories.html#ATOMIC_STAMPED_REFERENCE), [BIG_DECIMAL](InstanceOfAssertFactories.html#BIG_DECIMAL), [BIG_INTEGER](InstanceOfAssertFactories.html#BIG_INTEGER), [BOOLEAN](InstanceOfAssertFactories.html#BOOLEAN), [BOOLEAN_2D_ARRAY](InstanceOfAssertFactories.html#BOOLEAN_2D_ARRAY), [BOOLEAN_ARRAY](InstanceOfAssertFactories.html#BOOLEAN_ARRAY), [BYTE](InstanceOfAssertFactories.html#BYTE), [BYTE_2D_ARRAY](InstanceOfAssertFactories.html#BYTE_2D_ARRAY), [BYTE_ARRAY](InstanceOfAssertFactories.html#BYTE_ARRAY), [CHAR_2D_ARRAY](InstanceOfAssertFactories.html#CHAR_2D_ARRAY), [CHAR_ARRAY](InstanceOfAssertFactories.html#CHAR_ARRAY), [CHAR_SEQUENCE](InstanceOfAssertFactories.html#CHAR_SEQUENCE), [CHARACTER](InstanceOfAssertFactories.html#CHARACTER), [CLASS](InstanceOfAssertFactories.html#CLASS), [COLLECTION](InstanceOfAssertFactories.html#COLLECTION), [COMPLETABLE_FUTURE](InstanceOfAssertFactories.html#COMPLETABLE_FUTURE), [COMPLETION_STAGE](InstanceOfAssertFactories.html#COMPLETION_STAGE), [DATE](InstanceOfAssertFactories.html#DATE), [DOUBLE](InstanceOfAssertFactories.html#DOUBLE), [DOUBLE_2D_ARRAY](InstanceOfAssertFactories.html#DOUBLE_2D_ARRAY), [DOUBLE_ARRAY](InstanceOfAssertFactories.html#DOUBLE_ARRAY), [DOUBLE_PREDICATE](InstanceOfAssertFactories.html#DOUBLE_PREDICATE), [DOUBLE_STREAM](InstanceOfAssertFactories.html#DOUBLE_STREAM), [DURATION](InstanceOfAssertFactories.html#DURATION), [FILE](InstanceOfAssertFactories.html#FILE), [FLOAT](InstanceOfAssertFactories.html#FLOAT), [FLOAT_2D_ARRAY](InstanceOfAssertFactories.html#FLOAT_2D_ARRAY), [FLOAT_ARRAY](InstanceOfAssertFactories.html#FLOAT_ARRAY), [FUTURE](InstanceOfAssertFactories.html#FUTURE), [INPUT_STREAM](InstanceOfAssertFactories.html#INPUT_STREAM), [INSTANT](InstanceOfAssertFactories.html#INSTANT), [INT_2D_ARRAY](InstanceOfAssertFactories.html#INT_2D_ARRAY), [INT_ARRAY](InstanceOfAssertFactories.html#INT_ARRAY), [INT_PREDICATE](InstanceOfAssertFactories.html#INT_PREDICATE), [INT_STREAM](InstanceOfAssertFactories.html#INT_STREAM), [INTEGER](InstanceOfAssertFactories.html#INTEGER), [ITERABLE](InstanceOfAssertFactories.html#ITERABLE), [ITERATOR](InstanceOfAssertFactories.html#ITERATOR), [LIST](InstanceOfAssertFactories.html#LIST), [LOCAL_DATE](InstanceOfAssertFactories.html#LOCAL_DATE), [LOCAL_DATE_TIME](InstanceOfAssertFactories.html#LOCAL_DATE_TIME), [LOCAL_TIME](InstanceOfAssertFactories.html#LOCAL_TIME), [LONG](InstanceOfAssertFactories.html#LONG), [LONG_2D_ARRAY](InstanceOfAssertFactories.html#LONG_2D_ARRAY), [LONG_ADDER](InstanceOfAssertFactories.html#LONG_ADDER), [LONG_ARRAY](InstanceOfAssertFactories.html#LONG_ARRAY), [LONG_PREDICATE](InstanceOfAssertFactories.html#LONG_PREDICATE), [LONG_STREAM](InstanceOfAssertFactories.html#LONG_STREAM), [MAP](InstanceOfAssertFactories.html#MAP), [MATCHER](InstanceOfAssertFactories.html#MATCHER), [OFFSET_DATE_TIME](InstanceOfAssertFactories.html#OFFSET_DATE_TIME), [OFFSET_TIME](InstanceOfAssertFactories.html#OFFSET_TIME), [OPTIONAL](InstanceOfAssertFactories.html#OPTIONAL), [OPTIONAL_DOUBLE](InstanceOfAssertFactories.html#OPTIONAL_DOUBLE), [OPTIONAL_INT](InstanceOfAssertFactories.html#OPTIONAL_INT), [OPTIONAL_LONG](InstanceOfAssertFactories.html#OPTIONAL_LONG), [PATH](InstanceOfAssertFactories.html#PATH), [PERIOD](InstanceOfAssertFactories.html#PERIOD), [PREDICATE](InstanceOfAssertFactories.html#PREDICATE), [SHORT](InstanceOfAssertFactories.html#SHORT), [SHORT_2D_ARRAY](InstanceOfAssertFactories.html#SHORT_2D_ARRAY), [SHORT_ARRAY](InstanceOfAssertFactories.html#SHORT_ARRAY), [SPLITERATOR](InstanceOfAssertFactories.html#SPLITERATOR), [STREAM](InstanceOfAssertFactories.html#STREAM), [STRING](InstanceOfAssertFactories.html#STRING), [STRING_BUFFER](InstanceOfAssertFactories.html#STRING_BUFFER), [STRING_BUILDER](InstanceOfAssertFactories.html#STRING_BUILDER), [THROWABLE](InstanceOfAssertFactories.html#THROWABLE), [URI_TYPE](InstanceOfAssertFactories.html#URI_TYPE), [URL_TYPE](InstanceOfAssertFactories.html#URL_TYPE), [ZONED_DATE_TIME](InstanceOfAssertFactories.html#ZONED_DATE_TIME)`

* Constructor Summary
  ----------

  Constructors

  Modifier

  Constructor

  Description

  `protected `

  `[Assertions](#%3Cinit%3E())()`

  Creates a new `[`Assertions`](Assertions.html)`.

* Method Summary
  ----------

  All MethodsStatic MethodsConcrete Methods

  Modifier and Type

  Method

  Description

  `static <T> [Condition](Condition.html)<T>`

  `[allOf](#allOf(java.lang.Iterable))([Iterable](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html)<? extends [Condition](Condition.html)<? super T>> conditions)`

  Creates a new `[`AllOf`](../condition/AllOf.html)`

  `static <T> [Condition](Condition.html)<T>`

  `[allOf](#allOf(org.assertj.core.api.Condition...))([Condition](Condition.html)<? super T>... conditions)`

  Creates a new `[`AllOf`](../condition/AllOf.html)`

  `static <T> [ThrowingConsumer](ThrowingConsumer.html)<T>`

  `[allOf](#allOf(org.assertj.core.api.ThrowingConsumer...))([ThrowingConsumer](ThrowingConsumer.html)<? super T>... consumers)`

  Create a new `[`ThrowingConsumer`](ThrowingConsumer.html)` that delegates the evaluation of the given consumers to [`AbstractAssert.satisfies(ThrowingConsumer[])`](AbstractAssert.html#satisfies(org.assertj.core.api.ThrowingConsumer...)).

  `static <T> [Condition](Condition.html)<T>`

  `[anyOf](#anyOf(java.lang.Iterable))([Iterable](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html)<? extends [Condition](Condition.html)<? super T>> conditions)`

  Creates a new `[`AnyOf`](../condition/AnyOf.html)`

  `static <T> [Condition](Condition.html)<T>`

  `[anyOf](#anyOf(org.assertj.core.api.Condition...))([Condition](Condition.html)<? super T>... conditions)`

  Only delegate to [`AnyOf.anyOf(Condition...)`](../condition/AnyOf.html#anyOf(org.assertj.core.api.Condition...)) so that Assertions offers a full feature entry point to all AssertJ features (but you can use [`AnyOf`](../condition/AnyOf.html) if you prefer).

  `static <T> [ThrowingConsumer](ThrowingConsumer.html)<T>`

  `[anyOf](#anyOf(org.assertj.core.api.ThrowingConsumer...))([ThrowingConsumer](ThrowingConsumer.html)<? super T>... consumers)`

  Create a new `[`ThrowingConsumer`](ThrowingConsumer.html)` that delegates the evaluation of the given consumers to [`AbstractAssert.satisfiesAnyOf(ThrowingConsumer[])`](AbstractAssert.html#satisfiesAnyOf(org.assertj.core.api.ThrowingConsumer...)).

  `static <T,ASSERT extends [AbstractAssert](AbstractAssert.html)<?,?>>  
  [InstanceOfAssertFactory](InstanceOfAssertFactory.html)<T,ASSERT>`

  `[as](#as(org.assertj.core.api.InstanceOfAssertFactory))([InstanceOfAssertFactory](InstanceOfAssertFactory.html)<T,ASSERT> assertFactory)`

  A syntax sugar to write fluent assertion with methods having an [`InstanceOfAssertFactory`](InstanceOfAssertFactory.html) parameter.

  `static [AbstractBooleanAssert](AbstractBooleanAssert.html)<?>`

  `[assertThat](#assertThat(boolean))(boolean actual)`

  Creates a new instance of `[`BooleanAssert`](BooleanAssert.html)`.

  `static [AbstractBooleanArrayAssert](AbstractBooleanArrayAssert.html)<?>`

  `[assertThat](#assertThat(boolean%5B%5D))(boolean[] actual)`

  Creates a new instance of `[`BooleanArrayAssert`](BooleanArrayAssert.html)`.

  `static [Boolean2DArrayAssert](Boolean2DArrayAssert.html)`

  `[assertThat](#assertThat(boolean%5B%5D%5B%5D))(boolean[][] actual)`

  Creates a new instance of `[`Boolean2DArrayAssert`](Boolean2DArrayAssert.html)`.

  `static [AbstractByteAssert](AbstractByteAssert.html)<?>`

  `[assertThat](#assertThat(byte))(byte actual)`

  Creates a new instance of `[`ByteAssert`](ByteAssert.html)`.

  `static [AbstractByteArrayAssert](AbstractByteArrayAssert.html)<?>`

  `[assertThat](#assertThat(byte%5B%5D))(byte[] actual)`

  Creates a new instance of `[`ByteArrayAssert`](ByteArrayAssert.html)`.

  `static [Byte2DArrayAssert](Byte2DArrayAssert.html)`

  `[assertThat](#assertThat(byte%5B%5D%5B%5D))(byte[][] actual)`

  Creates a new instance of `[`Byte2DArrayAssert`](Byte2DArrayAssert.html)`.

  `static [AbstractCharacterAssert](AbstractCharacterAssert.html)<?>`

  `[assertThat](#assertThat(char))(char actual)`

  Creates a new instance of `[`CharacterAssert`](CharacterAssert.html)`.

  `static [AbstractCharArrayAssert](AbstractCharArrayAssert.html)<?>`

  `[assertThat](#assertThat(char%5B%5D))(char[] actual)`

  Creates a new instance of `[`CharArrayAssert`](CharArrayAssert.html)`.

  `static [Char2DArrayAssert](Char2DArrayAssert.html)`

  `[assertThat](#assertThat(char%5B%5D%5B%5D))(char[][] actual)`

  Creates a new instance of `[`CharArrayAssert`](CharArrayAssert.html)`.

  `static [AbstractDoubleAssert](AbstractDoubleAssert.html)<?>`

  `[assertThat](#assertThat(double))(double actual)`

  Creates a new instance of `[`DoubleAssert`](DoubleAssert.html)`.

  `static [AbstractDoubleArrayAssert](AbstractDoubleArrayAssert.html)<?>`

  `[assertThat](#assertThat(double%5B%5D))(double[] actual)`

  Creates a new instance of `[`DoubleArrayAssert`](DoubleArrayAssert.html)`.

  `static [Double2DArrayAssert](Double2DArrayAssert.html)`

  `[assertThat](#assertThat(double%5B%5D%5B%5D))(double[][] actual)`

  Creates a new instance of `[`Double2DArrayAssert`](Double2DArrayAssert.html)`.

  `static [AbstractFloatAssert](AbstractFloatAssert.html)<?>`

  `[assertThat](#assertThat(float))(float actual)`

  Creates a new instance of `[`FloatAssert`](FloatAssert.html)`.

  `static [AbstractFloatArrayAssert](AbstractFloatArrayAssert.html)<?>`

  `[assertThat](#assertThat(float%5B%5D))(float[] actual)`

  Creates a new instance of `[`FloatArrayAssert`](FloatArrayAssert.html)`.

  `static [Float2DArrayAssert](Float2DArrayAssert.html)`

  `[assertThat](#assertThat(float%5B%5D%5B%5D))(float[][] actual)`

  Creates a new instance of `[`Float2DArrayAssert`](Float2DArrayAssert.html)`.

  `static [AbstractIntegerAssert](AbstractIntegerAssert.html)<?>`

  `[assertThat](#assertThat(int))(int actual)`

  Creates a new instance of `[`IntegerAssert`](IntegerAssert.html)`.

  `static [AbstractIntArrayAssert](AbstractIntArrayAssert.html)<?>`

  `[assertThat](#assertThat(int%5B%5D))(int[] actual)`

  Creates a new instance of `[`IntArrayAssert`](IntArrayAssert.html)`.

  `static [Int2DArrayAssert](Int2DArrayAssert.html)`

  `[assertThat](#assertThat(int%5B%5D%5B%5D))(int[][] actual)`

  Creates a new instance of `[`Int2DArrayAssert`](Int2DArrayAssert.html)`.

  `static [AbstractLongAssert](AbstractLongAssert.html)<?>`

  `[assertThat](#assertThat(long))(long actual)`

  Creates a new instance of `[`LongAssert`](LongAssert.html)`.

  `static [AbstractLongArrayAssert](AbstractLongArrayAssert.html)<?>`

  `[assertThat](#assertThat(long%5B%5D))(long[] actual)`

  Creates a new instance of `[`LongArrayAssert`](LongArrayAssert.html)`.

  `static [Long2DArrayAssert](Long2DArrayAssert.html)`

  `[assertThat](#assertThat(long%5B%5D%5B%5D))(long[][] actual)`

  Creates a new instance of `[`Long2DArrayAssert`](Long2DArrayAssert.html)`.

  `static [AbstractShortAssert](AbstractShortAssert.html)<?>`

  `[assertThat](#assertThat(short))(short actual)`

  Creates a new instance of `[`ShortAssert`](ShortAssert.html)`.

  `static [AbstractShortArrayAssert](AbstractShortArrayAssert.html)<?>`

  `[assertThat](#assertThat(short%5B%5D))(short[] actual)`

  Creates a new instance of `[`ShortArrayAssert`](ShortArrayAssert.html)`.

  `static [Short2DArrayAssert](Short2DArrayAssert.html)`

  `[assertThat](#assertThat(short%5B%5D%5B%5D))(short[][] actual)`

  Creates a new instance of `[`Short2DArrayAssert`](Short2DArrayAssert.html)`.

  `static <ACTUAL extends [Iterable](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html)<? extends ELEMENT>,ELEMENT,ELEMENT_ASSERT extends [AbstractAssert](AbstractAssert.html)<ELEMENT_ASSERT,ELEMENT>>  
  [ClassBasedNavigableIterableAssert](ClassBasedNavigableIterableAssert.html)<?,ACTUAL,ELEMENT,ELEMENT_ASSERT>`

  `[assertThat](#assertThat(ACTUAL,java.lang.Class))(ACTUAL actual, [Class](https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html)<ELEMENT_ASSERT> assertClass)`

  Creates a new instance of `[`ClassBasedNavigableIterableAssert`](ClassBasedNavigableIterableAssert.html)` allowing to navigate to any `Iterable` element in order to perform assertions on it.

  `static [AbstractFileAssert](AbstractFileAssert.html)<?>`

  `[assertThat](#assertThat(java.io.File))([File](https://docs.oracle.com/javase/8/docs/api/java/io/File.html) actual)`

  Creates a new instance of `[`FileAssert`](FileAssert.html)`.

  `static [AbstractInputStreamAssert](AbstractInputStreamAssert.html)<?,? extends [InputStream](https://docs.oracle.com/javase/8/docs/api/java/io/InputStream.html)>`

  `[assertThat](#assertThat(java.io.InputStream))([InputStream](https://docs.oracle.com/javase/8/docs/api/java/io/InputStream.html) actual)`

  Creates a new instance of `[`InputStreamAssert`](InputStreamAssert.html)`.

  `static [AbstractBooleanAssert](AbstractBooleanAssert.html)<?>`

  `[assertThat](#assertThat(java.lang.Boolean))([Boolean](https://docs.oracle.com/javase/8/docs/api/java/lang/Boolean.html) actual)`

  Creates a new instance of `[`BooleanAssert`](BooleanAssert.html)`.

  `static [AbstractByteAssert](AbstractByteAssert.html)<?>`

  `[assertThat](#assertThat(java.lang.Byte))([Byte](https://docs.oracle.com/javase/8/docs/api/java/lang/Byte.html) actual)`

  Creates a new instance of `[`ByteAssert`](ByteAssert.html)`.

  `static [AbstractCharacterAssert](AbstractCharacterAssert.html)<?>`

  `[assertThat](#assertThat(java.lang.Character))([Character](https://docs.oracle.com/javase/8/docs/api/java/lang/Character.html) actual)`

  Creates a new instance of `[`CharacterAssert`](CharacterAssert.html)`.

  `static [AbstractCharSequenceAssert](AbstractCharSequenceAssert.html)<?,? extends [CharSequence](https://docs.oracle.com/javase/8/docs/api/java/lang/CharSequence.html)>`

  `[assertThat](#assertThat(java.lang.CharSequence))([CharSequence](https://docs.oracle.com/javase/8/docs/api/java/lang/CharSequence.html) actual)`

  Creates a new instance of `[`CharSequenceAssert`](CharSequenceAssert.html)`.

  `static [ClassAssert](ClassAssert.html)`

  `[assertThat](#assertThat(java.lang.Class))([Class](https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html)<?> actual)`

  Creates a new instance of `[`ClassAssert`](ClassAssert.html)`.

  `static [AbstractDoubleAssert](AbstractDoubleAssert.html)<?>`

  `[assertThat](#assertThat(java.lang.Double))([Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html) actual)`

  Creates a new instance of `[`DoubleAssert`](DoubleAssert.html)`.

  `static [AbstractFloatAssert](AbstractFloatAssert.html)<?>`

  `[assertThat](#assertThat(java.lang.Float))([Float](https://docs.oracle.com/javase/8/docs/api/java/lang/Float.html) actual)`

  Creates a new instance of `[`FloatAssert`](FloatAssert.html)`.

  `static [AbstractIntegerAssert](AbstractIntegerAssert.html)<?>`

  `[assertThat](#assertThat(java.lang.Integer))([Integer](https://docs.oracle.com/javase/8/docs/api/java/lang/Integer.html) actual)`

  Creates a new instance of `[`IntegerAssert`](IntegerAssert.html)`.

  `static <ELEMENT> [IterableAssert](IterableAssert.html)<ELEMENT>`

  `[assertThat](#assertThat(java.lang.Iterable))([Iterable](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html)<? extends ELEMENT> actual)`

  Creates a new instance of `[`IterableAssert`](IterableAssert.html)`.

  `static <ACTUAL extends [Iterable](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html)<? extends ELEMENT>,ELEMENT,ELEMENT_ASSERT extends [AbstractAssert](AbstractAssert.html)<ELEMENT_ASSERT,ELEMENT>>  
  [FactoryBasedNavigableIterableAssert](FactoryBasedNavigableIterableAssert.html)<?,ACTUAL,ELEMENT,ELEMENT_ASSERT>`

  `[assertThat](#assertThat(java.lang.Iterable,org.assertj.core.api.AssertFactory))([Iterable](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html)<? extends ELEMENT> actual, [AssertFactory](AssertFactory.html)<ELEMENT,ELEMENT_ASSERT> assertFactory)`

  Creates a new instance of `[`FactoryBasedNavigableIterableAssert`](FactoryBasedNavigableIterableAssert.html)` allowing to navigate to any `Iterable` element in order to perform assertions on it.

  `static [AbstractLongAssert](AbstractLongAssert.html)<?>`

  `[assertThat](#assertThat(java.lang.Long))([Long](https://docs.oracle.com/javase/8/docs/api/java/lang/Long.html) actual)`

  Creates a new instance of `[`LongAssert`](LongAssert.html)`.

  `static [AbstractShortAssert](AbstractShortAssert.html)<?>`

  `[assertThat](#assertThat(java.lang.Short))([Short](https://docs.oracle.com/javase/8/docs/api/java/lang/Short.html) actual)`

  Creates a new instance of `[`ShortAssert`](ShortAssert.html)`.

  `static [AbstractStringAssert](AbstractStringAssert.html)<?>`

  `[assertThat](#assertThat(java.lang.String))([String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) actual)`

  Creates a new instance of `[`StringAssert`](StringAssert.html)from a [`String`](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)`.

  `static [AbstractCharSequenceAssert](AbstractCharSequenceAssert.html)<?,? extends [CharSequence](https://docs.oracle.com/javase/8/docs/api/java/lang/CharSequence.html)>`

  `[assertThat](#assertThat(java.lang.StringBuffer))([StringBuffer](https://docs.oracle.com/javase/8/docs/api/java/lang/StringBuffer.html) actual)`

  Creates a new instance of `[`CharSequenceAssert`](CharSequenceAssert.html)` from a [`StringBuffer`](https://docs.oracle.com/javase/8/docs/api/java/lang/StringBuffer.html).

  `static [AbstractCharSequenceAssert](AbstractCharSequenceAssert.html)<?,? extends [CharSequence](https://docs.oracle.com/javase/8/docs/api/java/lang/CharSequence.html)>`

  `[assertThat](#assertThat(java.lang.StringBuilder))([StringBuilder](https://docs.oracle.com/javase/8/docs/api/java/lang/StringBuilder.html) actual)`

  Creates a new instance of `[`CharSequenceAssert`](CharSequenceAssert.html)` from a [`StringBuilder`](https://docs.oracle.com/javase/8/docs/api/java/lang/StringBuilder.html).

  `static [AbstractBigDecimalAssert](AbstractBigDecimalAssert.html)<?>`

  `[assertThat](#assertThat(java.math.BigDecimal))([BigDecimal](https://docs.oracle.com/javase/8/docs/api/java/math/BigDecimal.html) actual)`

  Creates a new instance of `[`BigDecimalAssert`](BigDecimalAssert.html)`.

  `static [AbstractBigIntegerAssert](AbstractBigIntegerAssert.html)<?>`

  `[assertThat](#assertThat(java.math.BigInteger))([BigInteger](https://docs.oracle.com/javase/8/docs/api/java/math/BigInteger.html) actual)`

  Creates a new instance of `[`BigIntegerAssert`](BigIntegerAssert.html)`.

  `static [AbstractUriAssert](AbstractUriAssert.html)<?>`

  `[assertThat](#assertThat(java.net.URI))([URI](https://docs.oracle.com/javase/8/docs/api/java/net/URI.html) actual)`

  Creates a new instance of `[`UriAssert`](UriAssert.html)`.

  `static [AbstractUrlAssert](AbstractUrlAssert.html)<?>`

  `[assertThat](#assertThat(java.net.URL))([URL](https://docs.oracle.com/javase/8/docs/api/java/net/URL.html) actual)`

  Creates a new instance of `[`UrlAssert`](UrlAssert.html)`.

  `static [AbstractPathAssert](AbstractPathAssert.html)<?>`

  `[assertThat](#assertThat(java.nio.file.Path))([Path](https://docs.oracle.com/javase/8/docs/api/java/nio/file/Path.html) actual)`

  Creates a new instance of [`PathAssert`](PathAssert.html)

  `static [AbstractDurationAssert](AbstractDurationAssert.html)<?>`

  `[assertThat](#assertThat(java.time.Duration))([Duration](https://docs.oracle.com/javase/8/docs/api/java/time/Duration.html) actual)`

  Creates a new instance of `[`DurationAssert`](DurationAssert.html)`.

  `static [AbstractInstantAssert](AbstractInstantAssert.html)<?>`

  `[assertThat](#assertThat(java.time.Instant))([Instant](https://docs.oracle.com/javase/8/docs/api/java/time/Instant.html) actual)`

  Creates a new instance of `[`InstantAssert`](InstantAssert.html)`.

  `static [AbstractLocalDateAssert](AbstractLocalDateAssert.html)<?>`

  `[assertThat](#assertThat(java.time.LocalDate))([LocalDate](https://docs.oracle.com/javase/8/docs/api/java/time/LocalDate.html) actual)`

  Creates a new instance of `[`LocalDateAssert`](LocalDateAssert.html)`.

  `static [AbstractLocalDateTimeAssert](AbstractLocalDateTimeAssert.html)<?>`

  `[assertThat](#assertThat(java.time.LocalDateTime))([LocalDateTime](https://docs.oracle.com/javase/8/docs/api/java/time/LocalDateTime.html) actual)`

  Creates a new instance of `[`LocalDateTimeAssert`](LocalDateTimeAssert.html)`.

  `static [AbstractLocalTimeAssert](AbstractLocalTimeAssert.html)<?>`

  `[assertThat](#assertThat(java.time.LocalTime))([LocalTime](https://docs.oracle.com/javase/8/docs/api/java/time/LocalTime.html) actual)`

  Creates a new instance of `[`LocalTimeAssert`](LocalTimeAssert.html)`.

  `static [AbstractOffsetDateTimeAssert](AbstractOffsetDateTimeAssert.html)<?>`

  `[assertThat](#assertThat(java.time.OffsetDateTime))([OffsetDateTime](https://docs.oracle.com/javase/8/docs/api/java/time/OffsetDateTime.html) actual)`

  Creates a new instance of `[`OffsetDateTime`](https://docs.oracle.com/javase/8/docs/api/java/time/OffsetDateTime.html)`.

  `static [AbstractOffsetTimeAssert](AbstractOffsetTimeAssert.html)<?>`

  `[assertThat](#assertThat(java.time.OffsetTime))([OffsetTime](https://docs.oracle.com/javase/8/docs/api/java/time/OffsetTime.html) actual)`

  Create assertion for [`OffsetTime`](https://docs.oracle.com/javase/8/docs/api/java/time/OffsetTime.html).

  `static [AbstractPeriodAssert](AbstractPeriodAssert.html)<?>`

  `[assertThat](#assertThat(java.time.Period))([Period](https://docs.oracle.com/javase/8/docs/api/java/time/Period.html) actual)`

  Creates a new instance of `[`PeriodAssert`](PeriodAssert.html)`.

  `static [AbstractZonedDateTimeAssert](AbstractZonedDateTimeAssert.html)<?>`

  `[assertThat](#assertThat(java.time.ZonedDateTime))([ZonedDateTime](https://docs.oracle.com/javase/8/docs/api/java/time/ZonedDateTime.html) actual)`

  Creates a new instance of `[`ZonedDateTimeAssert`](ZonedDateTimeAssert.html)`.

  `static <E> [AbstractCollectionAssert](AbstractCollectionAssert.html)<?,[Collection](https://docs.oracle.com/javase/8/docs/api/java/util/Collection.html)<? extends E>,E,[ObjectAssert](ObjectAssert.html)<E>>`

  `[assertThat](#assertThat(java.util.Collection))([Collection](https://docs.oracle.com/javase/8/docs/api/java/util/Collection.html)<? extends E> actual)`

  Creates a new instance of `[`CollectionAssert`](CollectionAssert.html)`.

  `static [AtomicBooleanAssert](AtomicBooleanAssert.html)`

  `[assertThat](#assertThat(java.util.concurrent.atomic.AtomicBoolean))([AtomicBoolean](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicBoolean.html) actual)`

  Create assertion for [`AtomicBoolean`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicBoolean.html).

  `static [AtomicIntegerAssert](AtomicIntegerAssert.html)`

  `[assertThat](#assertThat(java.util.concurrent.atomic.AtomicInteger))([AtomicInteger](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicInteger.html) actual)`

  Create assertion for [`AtomicInteger`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicInteger.html).

  `static [AtomicIntegerArrayAssert](AtomicIntegerArrayAssert.html)`

  `[assertThat](#assertThat(java.util.concurrent.atomic.AtomicIntegerArray))([AtomicIntegerArray](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicIntegerArray.html) actual)`

  Create int[] assertion for [`AtomicIntegerArray`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicIntegerArray.html).

  `static <OBJECT> [AtomicIntegerFieldUpdaterAssert](AtomicIntegerFieldUpdaterAssert.html)<OBJECT>`

  `[assertThat](#assertThat(java.util.concurrent.atomic.AtomicIntegerFieldUpdater))([AtomicIntegerFieldUpdater](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicIntegerFieldUpdater.html)<OBJECT> actual)`

  Create assertion for [`AtomicIntegerFieldUpdater`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicIntegerFieldUpdater.html).

  `static [AtomicLongAssert](AtomicLongAssert.html)`

  `[assertThat](#assertThat(java.util.concurrent.atomic.AtomicLong))([AtomicLong](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicLong.html) actual)`

  Create assertion for [`AtomicLong`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicLong.html).

  `static [AtomicLongArrayAssert](AtomicLongArrayAssert.html)`

  `[assertThat](#assertThat(java.util.concurrent.atomic.AtomicLongArray))([AtomicLongArray](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicLongArray.html) actual)`

  Create assertion for [`AtomicLongArray`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicLongArray.html).

  `static <OBJECT> [AtomicLongFieldUpdaterAssert](AtomicLongFieldUpdaterAssert.html)<OBJECT>`

  `[assertThat](#assertThat(java.util.concurrent.atomic.AtomicLongFieldUpdater))([AtomicLongFieldUpdater](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicLongFieldUpdater.html)<OBJECT> actual)`

  Create assertion for [`AtomicLongFieldUpdater`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicLongFieldUpdater.html).

  `static <VALUE> [AtomicMarkableReferenceAssert](AtomicMarkableReferenceAssert.html)<VALUE>`

  `[assertThat](#assertThat(java.util.concurrent.atomic.AtomicMarkableReference))([AtomicMarkableReference](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicMarkableReference.html)<VALUE> actual)`

  Create assertion for [`AtomicMarkableReference`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicMarkableReference.html).

  `static <VALUE> [AtomicReferenceAssert](AtomicReferenceAssert.html)<VALUE>`

  `[assertThat](#assertThat(java.util.concurrent.atomic.AtomicReference))([AtomicReference](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicReference.html)<VALUE> actual)`

  Create assertion for [`AtomicReference`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicReference.html).

  `static <ELEMENT> [AtomicReferenceArrayAssert](AtomicReferenceArrayAssert.html)<ELEMENT>`

  `[assertThat](#assertThat(java.util.concurrent.atomic.AtomicReferenceArray))([AtomicReferenceArray](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicReferenceArray.html)<ELEMENT> actual)`

  Create assertion for [`AtomicReferenceArray`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicReferenceArray.html).

  `static <FIELD,OBJECT>  
  [AtomicReferenceFieldUpdaterAssert](AtomicReferenceFieldUpdaterAssert.html)<FIELD,OBJECT>`

  `[assertThat](#assertThat(java.util.concurrent.atomic.AtomicReferenceFieldUpdater))([AtomicReferenceFieldUpdater](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicReferenceFieldUpdater.html)<OBJECT,FIELD> actual)`

  Create assertion for [`AtomicReferenceFieldUpdater`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicReferenceFieldUpdater.html).

  `static <VALUE> [AtomicStampedReferenceAssert](AtomicStampedReferenceAssert.html)<VALUE>`

  `[assertThat](#assertThat(java.util.concurrent.atomic.AtomicStampedReference))([AtomicStampedReference](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicStampedReference.html)<VALUE> actual)`

  Create assertion for [`AtomicStampedReference`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicStampedReference.html).

  `static [LongAdderAssert](LongAdderAssert.html)`

  `[assertThat](#assertThat(java.util.concurrent.atomic.LongAdder))([LongAdder](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/LongAdder.html) actual)`

  Create assertion for [`LongAdder`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/LongAdder.html).

  `static <RESULT> [CompletableFutureAssert](CompletableFutureAssert.html)<RESULT>`

  `[assertThat](#assertThat(java.util.concurrent.CompletableFuture))([CompletableFuture](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/CompletableFuture.html)<RESULT> actual)`

  Create assertion for [`CompletableFuture`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/CompletableFuture.html).

  `static <RESULT> [CompletableFutureAssert](CompletableFutureAssert.html)<RESULT>`

  `[assertThat](#assertThat(java.util.concurrent.CompletionStage))([CompletionStage](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/CompletionStage.html)<RESULT> actual)`

  Create assertion for [`CompletionStage`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/CompletionStage.html) by converting it to a [`CompletableFuture`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/CompletableFuture.html) and returning a [`CompletableFutureAssert`](CompletableFutureAssert.html).

  `static <RESULT> [FutureAssert](FutureAssert.html)<RESULT>`

  `[assertThat](#assertThat(java.util.concurrent.Future))([Future](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/Future.html)<RESULT> actual)`

  Create assertion for [`Future`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/Future.html).

  `static [AbstractDateAssert](AbstractDateAssert.html)<?>`

  `[assertThat](#assertThat(java.util.Date))([Date](https://docs.oracle.com/javase/8/docs/api/java/util/Date.html) actual)`

  Creates a new instance of `[`DateAssert`](DateAssert.html)`.

  `static [DoublePredicateAssert](DoublePredicateAssert.html)`

  `[assertThat](#assertThat(java.util.function.DoublePredicate))([DoublePredicate](https://docs.oracle.com/javase/8/docs/api/java/util/function/DoublePredicate.html) actual)`

  Create assertion for [`DoublePredicate`](https://docs.oracle.com/javase/8/docs/api/java/util/function/DoublePredicate.html).

  `static [IntPredicateAssert](IntPredicateAssert.html)`

  `[assertThat](#assertThat(java.util.function.IntPredicate))([IntPredicate](https://docs.oracle.com/javase/8/docs/api/java/util/function/IntPredicate.html) actual)`

  Create assertion for [`IntPredicate`](https://docs.oracle.com/javase/8/docs/api/java/util/function/IntPredicate.html).

  `static [LongPredicateAssert](LongPredicateAssert.html)`

  `[assertThat](#assertThat(java.util.function.LongPredicate))([LongPredicate](https://docs.oracle.com/javase/8/docs/api/java/util/function/LongPredicate.html) actual)`

  Create assertion for [`LongPredicate`](https://docs.oracle.com/javase/8/docs/api/java/util/function/LongPredicate.html).

  `static <T> [PredicateAssert](PredicateAssert.html)<T>`

  `[assertThat](#assertThat(java.util.function.Predicate))([Predicate](https://docs.oracle.com/javase/8/docs/api/java/util/function/Predicate.html)<T> actual)`

  Create assertion for [`Predicate`](https://docs.oracle.com/javase/8/docs/api/java/util/function/Predicate.html).

  `static <ELEMENT> [IteratorAssert](IteratorAssert.html)<ELEMENT>`

  `[assertThat](#assertThat(java.util.Iterator))([Iterator](https://docs.oracle.com/javase/8/docs/api/java/util/Iterator.html)<? extends ELEMENT> actual)`

  Creates a new instance of `[`IteratorAssert`](IteratorAssert.html)`.

  `static <ELEMENT> [ListAssert](ListAssert.html)<ELEMENT>`

  `[assertThat](#assertThat(java.util.List))([List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)<? extends ELEMENT> actual)`

  Creates a new instance of `[`ListAssert`](ListAssert.html)`.

  `static <ELEMENT,ACTUAL extends [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)<? extends ELEMENT>,ELEMENT_ASSERT extends [AbstractAssert](AbstractAssert.html)<ELEMENT_ASSERT,ELEMENT>>  
  [ClassBasedNavigableListAssert](ClassBasedNavigableListAssert.html)<?,ACTUAL,ELEMENT,ELEMENT_ASSERT>`

  `[assertThat](#assertThat(java.util.List,java.lang.Class))([List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)<? extends ELEMENT> actual, [Class](https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html)<ELEMENT_ASSERT> assertClass)`

  Creates a new instance of `[`ClassBasedNavigableListAssert`](ClassBasedNavigableListAssert.html)` allowing to navigate to any `List` element in order to perform assertions on it.

  `static <ACTUAL extends [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)<? extends ELEMENT>,ELEMENT,ELEMENT_ASSERT extends [AbstractAssert](AbstractAssert.html)<ELEMENT_ASSERT,ELEMENT>>  
  [FactoryBasedNavigableListAssert](FactoryBasedNavigableListAssert.html)<?,ACTUAL,ELEMENT,ELEMENT_ASSERT>`

  `[assertThat](#assertThat(java.util.List,org.assertj.core.api.AssertFactory))([List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)<? extends ELEMENT> actual, [AssertFactory](AssertFactory.html)<ELEMENT,ELEMENT_ASSERT> assertFactory)`

  Creates a new instance of `[`FactoryBasedNavigableListAssert`](FactoryBasedNavigableListAssert.html)` allowing to navigate to any `List` element in order to perform assertions on it.

  `static <K,V> [MapAssert](MapAssert.html)<K,V>`

  `[assertThat](#assertThat(java.util.Map))([Map](https://docs.oracle.com/javase/8/docs/api/java/util/Map.html)<K,V> actual)`

  Creates a new instance of `[`MapAssert`](MapAssert.html)`.

  `static <VALUE> [OptionalAssert](OptionalAssert.html)<VALUE>`

  `[assertThat](#assertThat(java.util.Optional))([Optional](https://docs.oracle.com/javase/8/docs/api/java/util/Optional.html)<VALUE> actual)`

  Create assertion for [`Optional`](https://docs.oracle.com/javase/8/docs/api/java/util/Optional.html).

  `static [OptionalDoubleAssert](OptionalDoubleAssert.html)`

  `[assertThat](#assertThat(java.util.OptionalDouble))([OptionalDouble](https://docs.oracle.com/javase/8/docs/api/java/util/OptionalDouble.html) actual)`

  Create assertion for [`OptionalDouble`](https://docs.oracle.com/javase/8/docs/api/java/util/OptionalDouble.html).

  `static [OptionalIntAssert](OptionalIntAssert.html)`

  `[assertThat](#assertThat(java.util.OptionalInt))([OptionalInt](https://docs.oracle.com/javase/8/docs/api/java/util/OptionalInt.html) actual)`

  Create assertion for [`OptionalInt`](https://docs.oracle.com/javase/8/docs/api/java/util/OptionalInt.html).

  `static [OptionalLongAssert](OptionalLongAssert.html)`

  `[assertThat](#assertThat(java.util.OptionalLong))([OptionalLong](https://docs.oracle.com/javase/8/docs/api/java/util/OptionalLong.html) actual)`

  Create assertion for [`OptionalInt`](https://docs.oracle.com/javase/8/docs/api/java/util/OptionalInt.html).

  `static [MatcherAssert](MatcherAssert.html)`

  `[assertThat](#assertThat(java.util.regex.Matcher))([Matcher](https://docs.oracle.com/javase/8/docs/api/java/util/regex/Matcher.html) actual)`

  Create assertion for [`Matcher`](https://docs.oracle.com/javase/8/docs/api/java/util/regex/Matcher.html).

  `static <ELEMENT> [SpliteratorAssert](SpliteratorAssert.html)<ELEMENT>`

  `[assertThat](#assertThat(java.util.Spliterator))([Spliterator](https://docs.oracle.com/javase/8/docs/api/java/util/Spliterator.html)<ELEMENT> actual)`

  Creates a new instance of `[`SpliteratorAssert`](SpliteratorAssert.html)` from the given [`Spliterator`](https://docs.oracle.com/javase/8/docs/api/java/util/Spliterator.html).

  `static [ListAssert](ListAssert.html)<[Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html)>`

  `[assertThat](#assertThat(java.util.stream.DoubleStream))([DoubleStream](https://docs.oracle.com/javase/8/docs/api/java/util/stream/DoubleStream.html) actual)`

  Creates a new instance of `[`ListAssert`](ListAssert.html)` from the given [`DoubleStream`](https://docs.oracle.com/javase/8/docs/api/java/util/stream/DoubleStream.html).

  `static [ListAssert](ListAssert.html)<[Integer](https://docs.oracle.com/javase/8/docs/api/java/lang/Integer.html)>`

  `[assertThat](#assertThat(java.util.stream.IntStream))([IntStream](https://docs.oracle.com/javase/8/docs/api/java/util/stream/IntStream.html) actual)`

  Creates a new instance of `[`ListAssert`](ListAssert.html)` from the given [`IntStream`](https://docs.oracle.com/javase/8/docs/api/java/util/stream/IntStream.html).

  `static [ListAssert](ListAssert.html)<[Long](https://docs.oracle.com/javase/8/docs/api/java/lang/Long.html)>`

  `[assertThat](#assertThat(java.util.stream.LongStream))([LongStream](https://docs.oracle.com/javase/8/docs/api/java/util/stream/LongStream.html) actual)`

  Creates a new instance of `[`ListAssert`](ListAssert.html)` from the given [`LongStream`](https://docs.oracle.com/javase/8/docs/api/java/util/stream/LongStream.html).

  `static <ELEMENT> [ListAssert](ListAssert.html)<ELEMENT>`

  `[assertThat](#assertThat(java.util.stream.Stream))([Stream](https://docs.oracle.com/javase/8/docs/api/java/util/stream/Stream.html)<? extends ELEMENT> actual)`

  Creates a new instance of `[`ListAssert`](ListAssert.html)` from the given [`Stream`](https://docs.oracle.com/javase/8/docs/api/java/util/stream/Stream.html).

  `static <T> T`

  `[assertThat](#assertThat(org.assertj.core.api.AssertProvider))([AssertProvider](AssertProvider.html)<T> component)`

  Delegates the creation of the [`Assert`](Assert.html) to the [`AssertProvider.assertThat()`](AssertProvider.html#assertThat()) of the given component.

  `static <T> [ObjectAssert](ObjectAssert.html)<T>`

  `[assertThat](#assertThat(T))(T actual)`

  Creates a new instance of `[`ObjectAssert`](ObjectAssert.html)`.

  `static <T> [ObjectArrayAssert](ObjectArrayAssert.html)<T>`

  `[assertThat](#assertThat(T%5B%5D))(T[] actual)`

  Creates a new instance of `[`ObjectArrayAssert`](ObjectArrayAssert.html)`.

  `static <T> [Object2DArrayAssert](Object2DArrayAssert.html)<T>`

  `[assertThat](#assertThat(T%5B%5D%5B%5D))(T[][] actual)`

  Creates a new instance of `[`Object2DArrayAssert`](Object2DArrayAssert.html)`.

  `static [AbstractCharSequenceAssert](AbstractCharSequenceAssert.html)<?,? extends [CharSequence](https://docs.oracle.com/javase/8/docs/api/java/lang/CharSequence.html)>`

  `[assertThatCharSequence](#assertThatCharSequence(java.lang.CharSequence))([CharSequence](https://docs.oracle.com/javase/8/docs/api/java/lang/CharSequence.html) actual)`

  Creates a new instance of `[`CharSequenceAssert`](CharSequenceAssert.html)`.

  `static [AbstractThrowableAssert](AbstractThrowableAssert.html)<?,? extends [Throwable](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html)>`

  `[assertThatCode](#assertThatCode(org.assertj.core.api.ThrowableAssert.ThrowingCallable))([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseOrNotThrowable)`

  Allows to capture and then assert on a [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) (easier done with lambdas).

  `static <E> [AbstractCollectionAssert](AbstractCollectionAssert.html)<?,[Collection](https://docs.oracle.com/javase/8/docs/api/java/util/Collection.html)<? extends E>,E,[ObjectAssert](ObjectAssert.html)<E>>`

  `[assertThatCollection](#assertThatCollection(java.util.Collection))([Collection](https://docs.oracle.com/javase/8/docs/api/java/util/Collection.html)<? extends E> actual)`

  Creates a new instance of `[`CollectionAssert`](CollectionAssert.html)`.

  `static <T> [AbstractUniversalComparableAssert](AbstractUniversalComparableAssert.html)<?,T>`

  `[assertThatComparable](#assertThatComparable(java.lang.Comparable))([Comparable](https://docs.oracle.com/javase/8/docs/api/java/lang/Comparable.html)<T> actual)`

  Creates a new instance of `[`UniversalComparableAssert`](UniversalComparableAssert.html)` with standard comparison semantics.

  `static [ThrowableTypeAssert](ThrowableTypeAssert.html)<[Exception](https://docs.oracle.com/javase/8/docs/api/java/lang/Exception.html)>`

  `[assertThatException](#assertThatException())()`

  Alias for [`assertThatExceptionOfType(Class)`](#assertThatExceptionOfType(java.lang.Class)) for [`Exception`](https://docs.oracle.com/javase/8/docs/api/java/lang/Exception.html).

  `static <T extends [Throwable](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html)>  
  [ThrowableTypeAssert](ThrowableTypeAssert.html)<T>`

  `[assertThatExceptionOfType](#assertThatExceptionOfType(java.lang.Class))([Class](https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html)<? extends T> exceptionType)`

  Entry point to check that an exception of type T is thrown by a given `throwingCallable` which allows to chain assertions on the thrown exception.

  `static [ThrowableTypeAssert](ThrowableTypeAssert.html)<[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)>`

  `[assertThatIllegalArgumentException](#assertThatIllegalArgumentException())()`

  Alias for [`assertThatExceptionOfType(Class)`](#assertThatExceptionOfType(java.lang.Class)) for [`IllegalArgumentException`](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html).

  `static [ThrowableTypeAssert](ThrowableTypeAssert.html)<[IllegalStateException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalStateException.html)>`

  `[assertThatIllegalStateException](#assertThatIllegalStateException())()`

  Alias for [`assertThatExceptionOfType(Class)`](#assertThatExceptionOfType(java.lang.Class)) for [`IllegalStateException`](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalStateException.html).

  `static [ThrowableTypeAssert](ThrowableTypeAssert.html)<[IndexOutOfBoundsException](https://docs.oracle.com/javase/8/docs/api/java/lang/IndexOutOfBoundsException.html)>`

  `[assertThatIndexOutOfBoundsException](#assertThatIndexOutOfBoundsException())()`

  Alias for [`assertThatExceptionOfType(Class)`](#assertThatExceptionOfType(java.lang.Class)) for [`IndexOutOfBoundsException`](https://docs.oracle.com/javase/8/docs/api/java/lang/IndexOutOfBoundsException.html).

  `static [ThrowableTypeAssert](ThrowableTypeAssert.html)<[IOException](https://docs.oracle.com/javase/8/docs/api/java/io/IOException.html)>`

  `[assertThatIOException](#assertThatIOException())()`

  Alias for [`assertThatExceptionOfType(Class)`](#assertThatExceptionOfType(java.lang.Class)) for [`IOException`](https://docs.oracle.com/javase/8/docs/api/java/io/IOException.html).

  `static <ELEMENT> [IterableAssert](IterableAssert.html)<ELEMENT>`

  `[assertThatIterable](#assertThatIterable(java.lang.Iterable))([Iterable](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html)<? extends ELEMENT> actual)`

  Creates a new instance of `[`IterableAssert`](IterableAssert.html)`.

  `static <ELEMENT> [IteratorAssert](IteratorAssert.html)<ELEMENT>`

  `[assertThatIterator](#assertThatIterator(java.util.Iterator))([Iterator](https://docs.oracle.com/javase/8/docs/api/java/util/Iterator.html)<? extends ELEMENT> actual)`

  Creates a new instance of `[`IteratorAssert`](IteratorAssert.html)`.

  `static <ELEMENT> [ListAssert](ListAssert.html)<ELEMENT>`

  `[assertThatList](#assertThatList(java.util.List))([List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)<? extends ELEMENT> actual)`

  Creates a new instance of `[`ListAssert`](ListAssert.html)`.

  `static [NotThrownAssert](NotThrownAssert.html)`

  `[assertThatNoException](#assertThatNoException())()`

  Entry point to check that no exception of any type is thrown by a given `throwingCallable`.

  `static [ThrowableTypeAssert](ThrowableTypeAssert.html)<[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)>`

  `[assertThatNullPointerException](#assertThatNullPointerException())()`

  Alias for [`assertThatExceptionOfType(Class)`](#assertThatExceptionOfType(java.lang.Class)) for [`NullPointerException`](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html).

  `static <T> [ObjectAssert](ObjectAssert.html)<T>`

  `[assertThatObject](#assertThatObject(T))(T actual)`

  Creates a new instance of `[`ObjectAssert`](ObjectAssert.html)` for any object.

  `static [AbstractPathAssert](AbstractPathAssert.html)<?>`

  `[assertThatPath](#assertThatPath(java.nio.file.Path))([Path](https://docs.oracle.com/javase/8/docs/api/java/nio/file/Path.html) actual)`

  Creates a new instance of [`PathAssert`](PathAssert.html)

  `static <T> [PredicateAssert](PredicateAssert.html)<T>`

  `[assertThatPredicate](#assertThatPredicate(java.util.function.Predicate))([Predicate](https://docs.oracle.com/javase/8/docs/api/java/util/function/Predicate.html)<T> actual)`

  Create assertion for [`Predicate`](https://docs.oracle.com/javase/8/docs/api/java/util/function/Predicate.html).

  `static [ThrowableTypeAssert](ThrowableTypeAssert.html)<[ReflectiveOperationException](https://docs.oracle.com/javase/8/docs/api/java/lang/ReflectiveOperationException.html)>`

  `[assertThatReflectiveOperationException](#assertThatReflectiveOperationException())()`

  Alias for [`assertThatExceptionOfType(Class)`](#assertThatExceptionOfType(java.lang.Class)) for [`ReflectiveOperationException`](https://docs.oracle.com/javase/8/docs/api/java/lang/ReflectiveOperationException.html).

  `static [ThrowableTypeAssert](ThrowableTypeAssert.html)<[RuntimeException](https://docs.oracle.com/javase/8/docs/api/java/lang/RuntimeException.html)>`

  `[assertThatRuntimeException](#assertThatRuntimeException())()`

  Alias for [`assertThatExceptionOfType(Class)`](#assertThatExceptionOfType(java.lang.Class)) for [`RuntimeException`](https://docs.oracle.com/javase/8/docs/api/java/lang/RuntimeException.html).

  `static <ELEMENT> [ListAssert](ListAssert.html)<ELEMENT>`

  `[assertThatStream](#assertThatStream(java.util.stream.Stream))([Stream](https://docs.oracle.com/javase/8/docs/api/java/util/stream/Stream.html)<? extends ELEMENT> actual)`

  Creates a new instance of `[`ListAssert`](ListAssert.html)` from the given [`Stream`](https://docs.oracle.com/javase/8/docs/api/java/util/stream/Stream.html).

  `static [AbstractThrowableAssert](AbstractThrowableAssert.html)<?,? extends [Throwable](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html)>`

  `[assertThatThrownBy](#assertThatThrownBy(org.assertj.core.api.ThrowableAssert.ThrowingCallable))([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseThrowable)`

  Allows to capture and then assert on a [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) (easier done with lambdas).

  `static [AbstractThrowableAssert](AbstractThrowableAssert.html)<?,? extends [Throwable](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html)>`

  `[assertThatThrownBy](#assertThatThrownBy(org.assertj.core.api.ThrowableAssert.ThrowingCallable,java.lang.String,java.lang.Object...))([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseThrowable, [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) description, [Object](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html)... args)`

  Allows to capture and then assert on a [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) like `assertThatThrownBy(ThrowingCallable)` but this method let you set the assertion description the same way you do with [`as(String, Object...)`](Descriptable.html#as(java.lang.String,java.lang.Object...)).

  `static <T> [ObjectAssert](ObjectAssert.html)<T>`

  `[assertWith](#assertWith(T,java.util.function.Consumer...))(T actual, [Consumer](https://docs.oracle.com/javase/8/docs/api/java/util/function/Consumer.html)<T>... requirements)`

  Uses the given instance as the instance under test for all the assertions expressed as the passed [`Consumer`](https://docs.oracle.com/javase/8/docs/api/java/util/function/Consumer.html).

  `static [Index](../data/Index.html)`

  `[atIndex](#atIndex(int))(int index)`

  Only delegate to [`Index.atIndex(int)`](../data/Index.html#atIndex(int)) so that Assertions offers a full feature entry point to all AssertJ features (but you can use [`Index`](../data/Index.html) if you prefer).

  `static [TemporalUnitOffset](../data/TemporalUnitOffset.html)`

  `[byLessThan](#byLessThan(long,java.time.temporal.TemporalUnit))(long value, [TemporalUnit](https://docs.oracle.com/javase/8/docs/api/java/time/temporal/TemporalUnit.html) unit)`

  Assertions entry point for [`TemporalUnitOffset`](../data/TemporalUnitOffset.html) with strict less than condition to use with `isCloseTo` temporal assertions.

  `static [Offset](../data/Offset.html)<[Byte](https://docs.oracle.com/javase/8/docs/api/java/lang/Byte.html)>`

  `[byLessThan](#byLessThan(java.lang.Byte))([Byte](https://docs.oracle.com/javase/8/docs/api/java/lang/Byte.html) value)`

  Assertions entry point for Byte [`Offset`](../data/Offset.html) to use with isCloseTo assertions.

  `static [Offset](../data/Offset.html)<[Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html)>`

  `[byLessThan](#byLessThan(java.lang.Double))([Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html) value)`

  Build a [`**strict** Offset`](../data/Offset.html#strictOffset(T)) to use with [`AbstractDoubleAssert.isCloseTo(double, Offset)`](AbstractDoubleAssert.html#isCloseTo(double,org.assertj.core.data.Offset)) and [`AbstractDoubleAssert.isNotCloseTo(double, Offset)`](AbstractDoubleAssert.html#isNotCloseTo(double,org.assertj.core.data.Offset)) assertions.

  `static [Offset](../data/Offset.html)<[Float](https://docs.oracle.com/javase/8/docs/api/java/lang/Float.html)>`

  `[byLessThan](#byLessThan(java.lang.Float))([Float](https://docs.oracle.com/javase/8/docs/api/java/lang/Float.html) value)`

  Alias for [`offset(Float)`](#offset(java.lang.Float)) to use with isCloseTo assertions.

  `static [Offset](../data/Offset.html)<[Integer](https://docs.oracle.com/javase/8/docs/api/java/lang/Integer.html)>`

  `[byLessThan](#byLessThan(java.lang.Integer))([Integer](https://docs.oracle.com/javase/8/docs/api/java/lang/Integer.html) value)`

  Assertions entry point for Long [`Offset`](../data/Offset.html) to use with strict [`isCloseTo`](AbstractIntegerAssert.html#isCloseTo(int,org.assertj.core.data.Offset)) assertions.

  `static [Offset](../data/Offset.html)<[Long](https://docs.oracle.com/javase/8/docs/api/java/lang/Long.html)>`

  `[byLessThan](#byLessThan(java.lang.Long))([Long](https://docs.oracle.com/javase/8/docs/api/java/lang/Long.html) value)`

  Assertions entry point for Long [`Offset`](../data/Offset.html) to use with strict [`isCloseTo`](AbstractLongAssert.html#isCloseTo(long,org.assertj.core.data.Offset)) assertions.

  `static [Offset](../data/Offset.html)<[Short](https://docs.oracle.com/javase/8/docs/api/java/lang/Short.html)>`

  `[byLessThan](#byLessThan(java.lang.Short))([Short](https://docs.oracle.com/javase/8/docs/api/java/lang/Short.html) value)`

  Assertions entry point for Short [`Offset`](../data/Offset.html) to use with isCloseTo assertions.

  `static [Offset](../data/Offset.html)<[BigDecimal](https://docs.oracle.com/javase/8/docs/api/java/math/BigDecimal.html)>`

  `[byLessThan](#byLessThan(java.math.BigDecimal))([BigDecimal](https://docs.oracle.com/javase/8/docs/api/java/math/BigDecimal.html) value)`

  Assertions entry point for BigDecimal [`Offset`](../data/Offset.html) to use with isCloseTo assertions.

  `static [Offset](../data/Offset.html)<[BigInteger](https://docs.oracle.com/javase/8/docs/api/java/math/BigInteger.html)>`

  `[byLessThan](#byLessThan(java.math.BigInteger))([BigInteger](https://docs.oracle.com/javase/8/docs/api/java/math/BigInteger.html) value)`

  Assertions entry point for BigInteger [`Offset`](../data/Offset.html) to use with isCloseTo assertions.

  `static [Exception](https://docs.oracle.com/javase/8/docs/api/java/lang/Exception.html)`

  `[catchException](#catchException(org.assertj.core.api.ThrowableAssert.ThrowingCallable))([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseException)`

  Allows catching an instance of [`Exception`](https://docs.oracle.com/javase/8/docs/api/java/lang/Exception.html).

  `static [IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)`

  `[catchIllegalArgumentException](#catchIllegalArgumentException(org.assertj.core.api.ThrowableAssert.ThrowingCallable))([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseIllegalArgumentException)`

  Allows catching an instance of [`IllegalArgumentException`](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html).

  `static [IllegalStateException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalStateException.html)`

  `[catchIllegalStateException](#catchIllegalStateException(org.assertj.core.api.ThrowableAssert.ThrowingCallable))([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseIllegalStateException)`

  Allows catching an instance of [`IllegalStateException`](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalStateException.html).

  `static [IndexOutOfBoundsException](https://docs.oracle.com/javase/8/docs/api/java/lang/IndexOutOfBoundsException.html)`

  `[catchIndexOutOfBoundsException](#catchIndexOutOfBoundsException(org.assertj.core.api.ThrowableAssert.ThrowingCallable))([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseIndexOutOfBoundException)`

  Allows catching an instance of [`IndexOutOfBoundsException`](https://docs.oracle.com/javase/8/docs/api/java/lang/IndexOutOfBoundsException.html).

  `static [IOException](https://docs.oracle.com/javase/8/docs/api/java/io/IOException.html)`

  `[catchIOException](#catchIOException(org.assertj.core.api.ThrowableAssert.ThrowingCallable))([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseIOException)`

  Allows catching an instance of [`IOException`](https://docs.oracle.com/javase/8/docs/api/java/io/IOException.html).

  `static [NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)`

  `[catchNullPointerException](#catchNullPointerException(org.assertj.core.api.ThrowableAssert.ThrowingCallable))([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseNullPointerException)`

  Allows catching an instance of [`NullPointerException`](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html).

  `static [ReflectiveOperationException](https://docs.oracle.com/javase/8/docs/api/java/lang/ReflectiveOperationException.html)`

  `[catchReflectiveOperationException](#catchReflectiveOperationException(org.assertj.core.api.ThrowableAssert.ThrowingCallable))([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseReflectiveOperationException)`

  Allows catching an instance of [`ReflectiveOperationException`](https://docs.oracle.com/javase/8/docs/api/java/lang/ReflectiveOperationException.html).

  `static [RuntimeException](https://docs.oracle.com/javase/8/docs/api/java/lang/RuntimeException.html)`

  `[catchRuntimeException](#catchRuntimeException(org.assertj.core.api.ThrowableAssert.ThrowingCallable))([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseRuntimeException)`

  Allows catching an instance of [`RuntimeException`](https://docs.oracle.com/javase/8/docs/api/java/lang/RuntimeException.html).

  `static [Throwable](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html)`

  `[catchThrowable](#catchThrowable(org.assertj.core.api.ThrowableAssert.ThrowingCallable))([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseThrowable)`

  Allows catching a [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) more easily when used with Java 8 lambdas.

  `static <THROWABLE extends [Throwable](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html)>  
  THROWABLE`

  `[catchThrowableOfType](#catchThrowableOfType(org.assertj.core.api.ThrowableAssert.ThrowingCallable,java.lang.Class))([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseThrowable, [Class](https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html)<THROWABLE> type)`

  Allows catching a [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) of a specific type.

  `static [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)`

  `[contentOf](#contentOf(java.io.File))([File](https://docs.oracle.com/javase/8/docs/api/java/io/File.html) file)`

  Loads the text content of a file with the default character set, so that it can be passed to [`assertThat(String)`](#assertThat(java.lang.String)).

  `static [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)`

  `[contentOf](#contentOf(java.io.File,java.lang.String))([File](https://docs.oracle.com/javase/8/docs/api/java/io/File.html) file, [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) charsetName)`

  Loads the text content of a file, so that it can be passed to [`assertThat(String)`](#assertThat(java.lang.String)).

  `static [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)`

  `[contentOf](#contentOf(java.io.File,java.nio.charset.Charset))([File](https://docs.oracle.com/javase/8/docs/api/java/io/File.html) file, [Charset](https://docs.oracle.com/javase/8/docs/api/java/nio/charset/Charset.html) charset)`

  Loads the text content of a file, so that it can be passed to [`assertThat(String)`](#assertThat(java.lang.String)).

  `static [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)`

  `[contentOf](#contentOf(java.net.URL))([URL](https://docs.oracle.com/javase/8/docs/api/java/net/URL.html) url)`

  Loads the text content of a URL with the default character set, so that it can be passed to [`assertThat(String)`](#assertThat(java.lang.String)).

  `static [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)`

  `[contentOf](#contentOf(java.net.URL,java.lang.String))([URL](https://docs.oracle.com/javase/8/docs/api/java/net/URL.html) url, [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) charsetName)`

  Loads the text content of a URL, so that it can be passed to [`assertThat(String)`](#assertThat(java.lang.String)).

  `static [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)`

  `[contentOf](#contentOf(java.net.URL,java.nio.charset.Charset))([URL](https://docs.oracle.com/javase/8/docs/api/java/net/URL.html) url, [Charset](https://docs.oracle.com/javase/8/docs/api/java/nio/charset/Charset.html) charset)`

  Loads the text content of a URL, so that it can be passed to [`assertThat(String)`](#assertThat(java.lang.String)).

  `static <T> [DoesNotHave](../condition/DoesNotHave.html)<T>`

  `[doesNotHave](#doesNotHave(org.assertj.core.api.Condition))([Condition](Condition.html)<? super T> condition)`

  Creates a new `[`DoesNotHave`](../condition/DoesNotHave.html)`.

  `static <K,V> [MapEntry](../data/MapEntry.html)<K,V>`

  `[entry](#entry(K,V))(K key, V value)`

  Only delegate to [`MapEntry.entry(Object, Object)`](../data/MapEntry.html#entry(K,V)) so that Assertions offers a full feature entry point to all AssertJ features (but you can use [`MapEntry`](../data/MapEntry.html) if you prefer).

  `static [Properties](../groups/Properties.html)<[Object](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html)>`

  `[extractProperty](#extractProperty(java.lang.String))([String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) propertyName)`

  Only delegate to [`Properties.extractProperty(String)`](../groups/Properties.html#extractProperty(java.lang.String)) so that Assertions offers a full feature entry point to all AssertJ features (but you can use [`Properties`](../groups/Properties.html) if you prefer).

  `static <T> [Properties](../groups/Properties.html)<T>`

  `[extractProperty](#extractProperty(java.lang.String,java.lang.Class))([String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) propertyName, [Class](https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html)<T> propertyType)`

  Only delegate to [`Properties.extractProperty(String)`](../groups/Properties.html#extractProperty(java.lang.String)) so that Assertions offers a full feature entry point to all AssertJ features (but you can use [`Properties`](../groups/Properties.html) if you prefer).

  `static <T> T`

  `[fail](#fail(java.lang.String))([String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) failureMessage)`

  Throws an [`AssertionError`](https://docs.oracle.com/javase/8/docs/api/java/lang/AssertionError.html) with the given message.

  `static <T> T`

  `[fail](#fail(java.lang.String,java.lang.Object...))([String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) failureMessage, [Object](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html)... args)`

  Throws an [`AssertionError`](https://docs.oracle.com/javase/8/docs/api/java/lang/AssertionError.html) with the given message built as [`String.format(String, Object...)`](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html#format-java.lang.String-java.lang.Object...-).

  `static <T> T`

  `[fail](#fail(java.lang.String,java.lang.Throwable))([String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) failureMessage, [Throwable](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) realCause)`

  Throws an [`AssertionError`](https://docs.oracle.com/javase/8/docs/api/java/lang/AssertionError.html) with the given message and with the [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) that caused the failure.

  `static <T> T`

  `[failBecauseExceptionWasNotThrown](#failBecauseExceptionWasNotThrown(java.lang.Class))([Class](https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html)<? extends [Throwable](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html)> throwableClass)`

  Throws an [`AssertionError`](https://docs.oracle.com/javase/8/docs/api/java/lang/AssertionError.html) with a message explaining that a [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) of given class was expected to be thrown but had not been.

  `static <E> [Filters](filter/Filters.html)<E>`

  `[filter](#filter(E%5B%5D))(E[] array)`

  Only delegate to [`Filters.filter(Object[])`](filter/Filters.html#filter(E%5B%5D)) so that Assertions offers a full feature entry point to all AssertJ features (but you can use [`Filters`](filter/Filters.html) if you prefer).

  `static <E> [Filters](filter/Filters.html)<E>`

  `[filter](#filter(java.lang.Iterable))([Iterable](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html)<E> iterableToFilter)`

  Only delegate to [`Filters.filter(Object[])`](filter/Filters.html#filter(E%5B%5D)) so that Assertions offers a full feature entry point to all AssertJ features (but you can use [`Filters`](filter/Filters.html) if you prefer).

  `static <F,T> [Function](https://docs.oracle.com/javase/8/docs/api/java/util/function/Function.html)<F,T>`

  `[from](#from(java.util.function.Function))([Function](https://docs.oracle.com/javase/8/docs/api/java/util/function/Function.html)<F,T> extractor)`

  A syntax sugar to write fluent assertion using [`AbstractObjectAssert.returns(Object, Function)`](AbstractObjectAssert.html#returns(T,java.util.function.Function)) and [`AbstractObjectAssert.doesNotReturn(Object, Function)`](AbstractObjectAssert.html#doesNotReturn(T,java.util.function.Function)).

  `static [InFilter](filter/InFilter.html)`

  `[in](#in(java.lang.Object...))([Object](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html)... values)`

  Create a [`FilterOperator`](filter/FilterOperator.html) to use in [`filteredOn(String, FilterOperation)`](AbstractIterableAssert.html#filteredOn(java.lang.String,org.assertj.core.api.filter.FilterOperator)) to express a filter keeping all Iterable elements whose property/field value matches one of the given values.

  `static [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)<[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)>`

  `[linesOf](#linesOf(java.io.File))([File](https://docs.oracle.com/javase/8/docs/api/java/io/File.html) file)`

  Loads the text content of a file into a list of strings with the default charset, each string corresponding to a line.

  `static [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)<[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)>`

  `[linesOf](#linesOf(java.io.File,java.lang.String))([File](https://docs.oracle.com/javase/8/docs/api/java/io/File.html) file, [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) charsetName)`

  Loads the text content of a file into a list of strings, each string corresponding to a line.

  `static [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)<[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)>`

  `[linesOf](#linesOf(java.io.File,java.nio.charset.Charset))([File](https://docs.oracle.com/javase/8/docs/api/java/io/File.html) file, [Charset](https://docs.oracle.com/javase/8/docs/api/java/nio/charset/Charset.html) charset)`

  Loads the text content of a file into a list of strings, each string corresponding to a line.

  `static [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)<[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)>`

  `[linesOf](#linesOf(java.net.URL))([URL](https://docs.oracle.com/javase/8/docs/api/java/net/URL.html) url)`

  Loads the text content of a URL into a list of strings with the default charset, each string corresponding to a line.

  `static [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)<[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)>`

  `[linesOf](#linesOf(java.net.URL,java.lang.String))([URL](https://docs.oracle.com/javase/8/docs/api/java/net/URL.html) url, [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) charsetName)`

  Loads the text content of a URL into a list of strings, each string corresponding to a line.

  `static [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)<[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)>`

  `[linesOf](#linesOf(java.net.URL,java.nio.charset.Charset))([URL](https://docs.oracle.com/javase/8/docs/api/java/net/URL.html) url, [Charset](https://docs.oracle.com/javase/8/docs/api/java/nio/charset/Charset.html) charset)`

  Loads the text content of a URL into a list of strings, each string corresponding to a line.

  `static [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)<[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)>`

  `[linesOf](#linesOf(java.nio.file.Path))([Path](https://docs.oracle.com/javase/8/docs/api/java/nio/file/Path.html) path)`

  Loads the text content of a file at a given path into a list of strings with the default charset, each string corresponding to a line.

  `static [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)<[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)>`

  `[linesOf](#linesOf(java.nio.file.Path,java.lang.String))([Path](https://docs.oracle.com/javase/8/docs/api/java/nio/file/Path.html) path, [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) charsetName)`

  Loads the text content of a file at a given path into a list of strings, each string corresponding to a line.

  `static [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)<[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)>`

  `[linesOf](#linesOf(java.nio.file.Path,java.nio.charset.Charset))([Path](https://docs.oracle.com/javase/8/docs/api/java/nio/file/Path.html) path, [Charset](https://docs.oracle.com/javase/8/docs/api/java/nio/charset/Charset.html) charset)`

  Loads the text content of a file at a given path into a list of strings, each string corresponding to a line.

  `static [NotFilter](filter/NotFilter.html)`

  `[not](#not(java.lang.Object))([Object](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html) valueNotToMatch)`

  Create a [`FilterOperator`](filter/FilterOperator.html) to use in [`filteredOn(String, FilterOperation)`](AbstractIterableAssert.html#filteredOn(java.lang.String,org.assertj.core.api.filter.FilterOperator)) to express a filter keeping all Iterable elements whose property/field value matches does not match the given value.

  `static <T> [Not](../condition/Not.html)<T>`

  `[not](#not(org.assertj.core.api.Condition))([Condition](Condition.html)<? super T> condition)`

  Creates a new `[`Not`](../condition/Not.html)`.

  `static [NotInFilter](filter/NotInFilter.html)`

  `[notIn](#notIn(java.lang.Object...))([Object](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html)... valuesNotToMatch)`

  Create a [`FilterOperator`](filter/FilterOperator.html) to use in [`filteredOn(String, FilterOperation)`](AbstractIterableAssert.html#filteredOn(java.lang.String,org.assertj.core.api.filter.FilterOperator)) to express a filter keeping all Iterable elements whose property/field value matches does not match any of the given values.

  `static [Offset](../data/Offset.html)<[Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html)>`

  `[offset](#offset(java.lang.Double))([Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html) value)`

  Assertions entry point for double [`Offset`](../data/Offset.html).

  `static [Offset](../data/Offset.html)<[Float](https://docs.oracle.com/javase/8/docs/api/java/lang/Float.html)>`

  `[offset](#offset(java.lang.Float))([Float](https://docs.oracle.com/javase/8/docs/api/java/lang/Float.html) value)`

  Assertions entry point for float [`Offset`](../data/Offset.html).

  `static void`

  `[registerCustomDateFormat](#registerCustomDateFormat(java.lang.String))([String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) userCustomDateFormatPattern)`

  Add the given date format to the ones used to parse date String in String based Date assertions like [`AbstractDateAssert.isEqualTo(String)`](AbstractDateAssert.html#isEqualTo(java.lang.String)).

  `static void`

  `[registerCustomDateFormat](#registerCustomDateFormat(java.text.DateFormat))([DateFormat](https://docs.oracle.com/javase/8/docs/api/java/text/DateFormat.html) userCustomDateFormat)`

  Add the given date format to the ones used to parse date String in String based Date assertions like [`AbstractDateAssert.isEqualTo(String)`](AbstractDateAssert.html#isEqualTo(java.lang.String)).

  `static <T> void`

  `[registerFormatterForType](#registerFormatterForType(java.lang.Class,java.util.function.Function))([Class](https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html)<T> type, [Function](https://docs.oracle.com/javase/8/docs/api/java/util/function/Function.html)<T,[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)> formatter)`

  Assertions error messages uses a [`Representation`](../presentation/Representation.html) to format the different types involved, using this method you can control the formatting of a given type by providing a specific formatter.

  `static void`

  `[setAllowComparingPrivateFields](#setAllowComparingPrivateFields(boolean))(boolean allowComparingPrivateFields)`

  Globally sets whether the use of private fields is allowed for comparison.

  `static void`

  `[setAllowExtractingPrivateFields](#setAllowExtractingPrivateFields(boolean))(boolean allowExtractingPrivateFields)`

  Globally sets whether `[`IterableAssert#extracting(String)`](AbstractIterableAssert.html#extracting(java.lang.String))` and `[`ObjectArrayAssert#extracting(String)`](AbstractObjectArrayAssert.html#extracting(java.lang.String))` should be allowed to extract private fields, if not and they try it fails with exception.

  `static void`

  `[setDescriptionConsumer](#setDescriptionConsumer(java.util.function.Consumer))([Consumer](https://docs.oracle.com/javase/8/docs/api/java/util/function/Consumer.html)<[Description](../description/Description.html)> descriptionConsumer)`

  All assertions description will be consumed by the given [`Consumer<Description>`](https://docs.oracle.com/javase/8/docs/api/java/util/function/Consumer.html) allowing for example to record them in a file.

  `static void`

  `[setExtractBareNamePropertyMethods](#setExtractBareNamePropertyMethods(boolean))(boolean barenamePropertyMethods)`

  Globally sets whether the extractor considers bare-named property methods like `String name()`.

  `static void`

  `[setLenientDateParsing](#setLenientDateParsing(boolean))(boolean value)`

  Instead of using default strict date/time parsing, it is possible to use lenient parsing mode for default date formats parser to interpret inputs that do not precisely match supported date formats (lenient parsing).

  `static void`

  `[setMaxElementsForPrinting](#setMaxElementsForPrinting(int))(int maxElementsForPrinting)`

  Sets the maximum number of elements to display in error messages for iterables, arrays and map .

  `static void`

  `[setMaxLengthForSingleLineDescription](#setMaxLengthForSingleLineDescription(int))(int maxLengthForSingleLineDescription)`

  In error messages, sets the threshold when iterable/array formatting will be on one line (if their String description length \<= this parameter) or it will be formatted with one element per line.

  `static void`

  `[setMaxStackTraceElementsDisplayed](#setMaxStackTraceElementsDisplayed(int))(int maxStackTraceElementsDisplayed)`

  Sets how many stacktrace elements are included in [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) representation (by default this set to 3).

  `static void`

  `[setPrintAssertionsDescription](#setPrintAssertionsDescription(boolean))(boolean printAssertionsDescription)`

  Enable/disable printing assertions description to the console (disabled by default).

  `static void`

  `[setRemoveAssertJRelatedElementsFromStackTrace](#setRemoveAssertJRelatedElementsFromStackTrace(boolean))(boolean removeAssertJRelatedElementsFromStackTrace)`

  Sets whether we remove elements related to AssertJ from assertion error stack trace.

  `static <T> T`

  `[shouldHaveThrown](#shouldHaveThrown(java.lang.Class))([Class](https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html)<? extends [Throwable](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html)> throwableClass)`

  Throws an [`AssertionError`](https://docs.oracle.com/javase/8/docs/api/java/lang/AssertionError.html) with a message explaining that a [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) of given class was expected to be thrown but had not been.

  `static [Tuple](../groups/Tuple.html)`

  `[tuple](#tuple(java.lang.Object...))([Object](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html)... values)`

  Utility method to build nicely a [`Tuple`](../groups/Tuple.html) when working with [`AbstractIterableAssert.extracting(String...)`](AbstractIterableAssert.html#extracting(java.lang.String...)) or [`AbstractObjectArrayAssert.extracting(String...)`](AbstractObjectArrayAssert.html#extracting(java.lang.String...))

  `static void`

  `[useDefaultDateFormatsOnly](#useDefaultDateFormatsOnly())()`

  Remove all registered custom date formats =\> use only the defaults date formats to parse string as date.

  `static void`

  `[useDefaultRepresentation](#useDefaultRepresentation())()`

  Reset the representaion being used to the one from the [`Configuration`](../configuration/Configuration.html) to revert the effect of calling [`useRepresentation(Representation)`](#useRepresentation(org.assertj.core.presentation.Representation)).

  `static void`

  `[useRepresentation](#useRepresentation(org.assertj.core.presentation.Representation))([Representation](../presentation/Representation.html) customRepresentation)`

  Use the given [`Representation`](../presentation/Representation.html) in all remaining tests assertions.

  `static [TemporalUnitOffset](../data/TemporalUnitOffset.html)`

  `[within](#within(long,java.time.temporal.TemporalUnit))(long value, [TemporalUnit](https://docs.oracle.com/javase/8/docs/api/java/time/temporal/TemporalUnit.html) unit)`

  Assertions entry point for [`TemporalUnitOffset`](../data/TemporalUnitOffset.html) with less than or equal condition to use with isCloseTo temporal assertions.

  `static [Offset](../data/Offset.html)<[Byte](https://docs.oracle.com/javase/8/docs/api/java/lang/Byte.html)>`

  `[within](#within(java.lang.Byte))([Byte](https://docs.oracle.com/javase/8/docs/api/java/lang/Byte.html) value)`

  Assertions entry point for Byte [`Offset`](../data/Offset.html) to use with isCloseTo assertions.

  `static [Offset](../data/Offset.html)<[Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html)>`

  `[within](#within(java.lang.Double))([Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html) value)`

  Alias for [`offset(Double)`](#offset(java.lang.Double)) to use with isCloseTo assertions.

  `static [Offset](../data/Offset.html)<[Float](https://docs.oracle.com/javase/8/docs/api/java/lang/Float.html)>`

  `[within](#within(java.lang.Float))([Float](https://docs.oracle.com/javase/8/docs/api/java/lang/Float.html) value)`

  Alias for [`offset(Float)`](#offset(java.lang.Float)) to use with isCloseTo assertions.

  `static [Offset](../data/Offset.html)<[Integer](https://docs.oracle.com/javase/8/docs/api/java/lang/Integer.html)>`

  `[within](#within(java.lang.Integer))([Integer](https://docs.oracle.com/javase/8/docs/api/java/lang/Integer.html) value)`

  Assertions entry point for Integer [`Offset`](../data/Offset.html) to use with isCloseTo assertions.

  `static [Offset](../data/Offset.html)<[Long](https://docs.oracle.com/javase/8/docs/api/java/lang/Long.html)>`

  `[within](#within(java.lang.Long))([Long](https://docs.oracle.com/javase/8/docs/api/java/lang/Long.html) value)`

  Assertions entry point for Long [`Offset`](../data/Offset.html) to use with [`isCloseTo`](AbstractLongAssert.html#isCloseTo(long,org.assertj.core.data.Offset)) assertions.

  `static [Offset](../data/Offset.html)<[Short](https://docs.oracle.com/javase/8/docs/api/java/lang/Short.html)>`

  `[within](#within(java.lang.Short))([Short](https://docs.oracle.com/javase/8/docs/api/java/lang/Short.html) value)`

  Assertions entry point for Short [`Offset`](../data/Offset.html) to use with isCloseTo assertions.

  `static [Offset](../data/Offset.html)<[BigDecimal](https://docs.oracle.com/javase/8/docs/api/java/math/BigDecimal.html)>`

  `[within](#within(java.math.BigDecimal))([BigDecimal](https://docs.oracle.com/javase/8/docs/api/java/math/BigDecimal.html) value)`

  Assertions entry point for BigDecimal [`Offset`](../data/Offset.html) to use with isCloseTo assertions.

  `static [Offset](../data/Offset.html)<[BigInteger](https://docs.oracle.com/javase/8/docs/api/java/math/BigInteger.html)>`

  `[within](#within(java.math.BigInteger))([BigInteger](https://docs.oracle.com/javase/8/docs/api/java/math/BigInteger.html) value)`

  Assertions entry point for BigInteger [`Offset`](../data/Offset.html) to use with isCloseTo assertions.

  `static [Percentage](../data/Percentage.html)`

  `[withinPercentage](#withinPercentage(java.lang.Double))([Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html) value)`

  Assertions entry point for Double [`Percentage`](../data/Percentage.html) to use with isCloseTo assertions for percentages.

  `static [Percentage](../data/Percentage.html)`

  `[withinPercentage](#withinPercentage(java.lang.Integer))([Integer](https://docs.oracle.com/javase/8/docs/api/java/lang/Integer.html) value)`

  Assertions entry point for Integer [`Percentage`](../data/Percentage.html) to use with isCloseTo assertions for percentages.

  `static [Percentage](../data/Percentage.html)`

  `[withinPercentage](#withinPercentage(java.lang.Long))([Long](https://docs.oracle.com/javase/8/docs/api/java/lang/Long.html) value)`

  Assertions entry point for Long [`Percentage`](../data/Percentage.html) to use with isCloseTo assertions for percentages.

  `static [Duration](https://docs.oracle.com/javase/8/docs/api/java/time/Duration.html)`

  `[withMarginOf](#withMarginOf(java.time.Duration))([Duration](https://docs.oracle.com/javase/8/docs/api/java/time/Duration.html) allowedDifference)`

  Syntactic sugar method to use with [`AbstractDurationAssert.isCloseTo(Duration, Duration)`](AbstractDurationAssert.html#isCloseTo(java.time.Duration,java.time.Duration)) assertion.

  `static [Offset](../data/Offset.html)<[Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html)>`

  `[withPrecision](#withPrecision(java.lang.Double))([Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html) value)`

  Alias for [`offset(Double)`](#offset(java.lang.Double)) to use with real number assertions.

  `static [Offset](../data/Offset.html)<[Float](https://docs.oracle.com/javase/8/docs/api/java/lang/Float.html)>`

  `[withPrecision](#withPrecision(java.lang.Float))([Float](https://docs.oracle.com/javase/8/docs/api/java/lang/Float.html) value)`

  Alias for [`offset(Float)`](#offset(java.lang.Float)) to use with real number assertions.

  ### Methods inherited from class java.lang.[Object](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html) ###

  `[clone](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html#clone--), [equals](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html#equals-java.lang.Object-), [finalize](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html#finalize--), [getClass](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html#getClass--), [hashCode](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html#hashCode--), [notify](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html#notify--), [notifyAll](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html#notifyAll--), [toString](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html#toString--), [wait](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html#wait--), [wait](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html#wait-long-), [wait](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html#wait-long-int-)`

* Constructor Details
  ----------

  * ### Assertions ###

    protected Assertions()

    Creates a new `[`Assertions`](Assertions.html)`.

* Method Details
  ----------

  * ### assertThat ###

    public static \<T\> [PredicateAssert](PredicateAssert.html)\<T\> assertThat([Predicate](https://docs.oracle.com/javase/8/docs/api/java/util/function/Predicate.html)\<T\> actual)

    Create assertion for [`Predicate`](https://docs.oracle.com/javase/8/docs/api/java/util/function/Predicate.html).

    Type Parameters:`T` - the type of the value contained in the [`Predicate`](https://docs.oracle.com/javase/8/docs/api/java/util/function/Predicate.html).Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.5.0

  * ### assertThatPredicate ###

    public static \<T\> [PredicateAssert](PredicateAssert.html)\<T\> assertThatPredicate([Predicate](https://docs.oracle.com/javase/8/docs/api/java/util/function/Predicate.html)\<T\> actual)

    Create assertion for [`Predicate`](https://docs.oracle.com/javase/8/docs/api/java/util/function/Predicate.html).

     Use this over [`assertThat(Predicate)`](#assertThat(java.util.function.Predicate)) in case of ambiguous method resolution when the object under test implements several interfaces Assertj provides `assertThat` for.

    Type Parameters:`T` - the type of the value contained in the [`Predicate`](https://docs.oracle.com/javase/8/docs/api/java/util/function/Predicate.html).Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.23.0

  * ### assertThat ###

    public static [IntPredicateAssert](IntPredicateAssert.html) assertThat([IntPredicate](https://docs.oracle.com/javase/8/docs/api/java/util/function/IntPredicate.html) actual)

    Create assertion for [`IntPredicate`](https://docs.oracle.com/javase/8/docs/api/java/util/function/IntPredicate.html).

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.5.0

  * ### assertThat ###

    public static [LongPredicateAssert](LongPredicateAssert.html) assertThat([LongPredicate](https://docs.oracle.com/javase/8/docs/api/java/util/function/LongPredicate.html) actual)

    Create assertion for [`LongPredicate`](https://docs.oracle.com/javase/8/docs/api/java/util/function/LongPredicate.html).

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.5.0

  * ### assertThat ###

    public static [DoublePredicateAssert](DoublePredicateAssert.html) assertThat([DoublePredicate](https://docs.oracle.com/javase/8/docs/api/java/util/function/DoublePredicate.html) actual)

    Create assertion for [`DoublePredicate`](https://docs.oracle.com/javase/8/docs/api/java/util/function/DoublePredicate.html).

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.5.0

  * ### assertThat ###

    public static \<RESULT\>[CompletableFutureAssert](CompletableFutureAssert.html)\<RESULT\> assertThat([CompletableFuture](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/CompletableFuture.html)\<RESULT\> actual)

    Create assertion for [`CompletableFuture`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/CompletableFuture.html).

    Type Parameters:`RESULT` - the type of the value contained in the [`CompletableFuture`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/CompletableFuture.html).Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static \<RESULT\>[CompletableFutureAssert](CompletableFutureAssert.html)\<RESULT\> assertThat([CompletionStage](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/CompletionStage.html)\<RESULT\> actual)

    Create assertion for [`CompletionStage`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/CompletionStage.html) by converting it to a [`CompletableFuture`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/CompletableFuture.html) and returning a [`CompletableFutureAssert`](CompletableFutureAssert.html).

     If the given [`CompletionStage`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/CompletionStage.html) is null, the [`CompletableFuture`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/CompletableFuture.html) in the returned [`CompletableFutureAssert`](CompletableFutureAssert.html) will also be null.

    Type Parameters:`RESULT` - the type of the value contained in the [`CompletionStage`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/CompletionStage.html).Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static \<VALUE\> [OptionalAssert](OptionalAssert.html)\<VALUE\> assertThat([Optional](https://docs.oracle.com/javase/8/docs/api/java/util/Optional.html)\<VALUE\> actual)

    Create assertion for [`Optional`](https://docs.oracle.com/javase/8/docs/api/java/util/Optional.html).

    Type Parameters:`VALUE` - the type of the value contained in the [`Optional`](https://docs.oracle.com/javase/8/docs/api/java/util/Optional.html).Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [OptionalDoubleAssert](OptionalDoubleAssert.html) assertThat([OptionalDouble](https://docs.oracle.com/javase/8/docs/api/java/util/OptionalDouble.html) actual)

    Create assertion for [`OptionalDouble`](https://docs.oracle.com/javase/8/docs/api/java/util/OptionalDouble.html).

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [OptionalIntAssert](OptionalIntAssert.html) assertThat([OptionalInt](https://docs.oracle.com/javase/8/docs/api/java/util/OptionalInt.html) actual)

    Create assertion for [`OptionalInt`](https://docs.oracle.com/javase/8/docs/api/java/util/OptionalInt.html).

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [OptionalLongAssert](OptionalLongAssert.html) assertThat([OptionalLong](https://docs.oracle.com/javase/8/docs/api/java/util/OptionalLong.html) actual)

    Create assertion for [`OptionalInt`](https://docs.oracle.com/javase/8/docs/api/java/util/OptionalInt.html).

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [MatcherAssert](MatcherAssert.html) assertThat([Matcher](https://docs.oracle.com/javase/8/docs/api/java/util/regex/Matcher.html) actual)

    Create assertion for [`Matcher`](https://docs.oracle.com/javase/8/docs/api/java/util/regex/Matcher.html).

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractBigDecimalAssert](AbstractBigDecimalAssert.html)\<?\> assertThat([BigDecimal](https://docs.oracle.com/javase/8/docs/api/java/math/BigDecimal.html) actual)

    Creates a new instance of `[`BigDecimalAssert`](BigDecimalAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractBigIntegerAssert](AbstractBigIntegerAssert.html)\<?\> assertThat([BigInteger](https://docs.oracle.com/javase/8/docs/api/java/math/BigInteger.html) actual)

    Creates a new instance of `[`BigIntegerAssert`](BigIntegerAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:2.7.0 / 3.7.0

  * ### assertThat ###

    public static [AbstractUriAssert](AbstractUriAssert.html)\<?\> assertThat([URI](https://docs.oracle.com/javase/8/docs/api/java/net/URI.html) actual)

    Creates a new instance of `[`UriAssert`](UriAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractUrlAssert](AbstractUrlAssert.html)\<?\> assertThat([URL](https://docs.oracle.com/javase/8/docs/api/java/net/URL.html) actual)

    Creates a new instance of `[`UrlAssert`](UrlAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractBooleanAssert](AbstractBooleanAssert.html)\<?\> assertThat(boolean actual)

    Creates a new instance of `[`BooleanAssert`](BooleanAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractBooleanAssert](AbstractBooleanAssert.html)\<?\> assertThat([Boolean](https://docs.oracle.com/javase/8/docs/api/java/lang/Boolean.html) actual)

    Creates a new instance of `[`BooleanAssert`](BooleanAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractBooleanArrayAssert](AbstractBooleanArrayAssert.html)\<?\> assertThat(boolean[] actual)

    Creates a new instance of `[`BooleanArrayAssert`](BooleanArrayAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [Boolean2DArrayAssert](Boolean2DArrayAssert.html) assertThat(boolean[][] actual)

    Creates a new instance of `[`Boolean2DArrayAssert`](Boolean2DArrayAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.17.0

  * ### assertThat ###

    public static [AbstractByteAssert](AbstractByteAssert.html)\<?\> assertThat(byte actual)

    Creates a new instance of `[`ByteAssert`](ByteAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractByteAssert](AbstractByteAssert.html)\<?\> assertThat([Byte](https://docs.oracle.com/javase/8/docs/api/java/lang/Byte.html) actual)

    Creates a new instance of `[`ByteAssert`](ByteAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractByteArrayAssert](AbstractByteArrayAssert.html)\<?\> assertThat(byte[] actual)

    Creates a new instance of `[`ByteArrayAssert`](ByteArrayAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [Byte2DArrayAssert](Byte2DArrayAssert.html) assertThat(byte[][] actual)

    Creates a new instance of `[`Byte2DArrayAssert`](Byte2DArrayAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.17.0

  * ### assertThat ###

    public static [AbstractCharacterAssert](AbstractCharacterAssert.html)\<?\> assertThat(char actual)

    Creates a new instance of `[`CharacterAssert`](CharacterAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractCharArrayAssert](AbstractCharArrayAssert.html)\<?\> assertThat(char[] actual)

    Creates a new instance of `[`CharArrayAssert`](CharArrayAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [Char2DArrayAssert](Char2DArrayAssert.html) assertThat(char[][] actual)

    Creates a new instance of `[`CharArrayAssert`](CharArrayAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.17.0

  * ### assertThat ###

    public static [AbstractCharacterAssert](AbstractCharacterAssert.html)\<?\> assertThat([Character](https://docs.oracle.com/javase/8/docs/api/java/lang/Character.html) actual)

    Creates a new instance of `[`CharacterAssert`](CharacterAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [ClassAssert](ClassAssert.html) assertThat([Class](https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html)\<?\> actual)

    Creates a new instance of `[`ClassAssert`](ClassAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractDoubleAssert](AbstractDoubleAssert.html)\<?\> assertThat(double actual)

    Creates a new instance of `[`DoubleAssert`](DoubleAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractDoubleAssert](AbstractDoubleAssert.html)\<?\> assertThat([Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html) actual)

    Creates a new instance of `[`DoubleAssert`](DoubleAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractDoubleArrayAssert](AbstractDoubleArrayAssert.html)\<?\> assertThat(double[] actual)

    Creates a new instance of `[`DoubleArrayAssert`](DoubleArrayAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [Double2DArrayAssert](Double2DArrayAssert.html) assertThat(double[][] actual)

    Creates a new instance of `[`Double2DArrayAssert`](Double2DArrayAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.17.0

  * ### assertThat ###

    public static [AbstractFileAssert](AbstractFileAssert.html)\<?\> assertThat([File](https://docs.oracle.com/javase/8/docs/api/java/io/File.html) actual)

    Creates a new instance of `[`FileAssert`](FileAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static \<RESULT\> [FutureAssert](FutureAssert.html)\<RESULT\> assertThat([Future](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/Future.html)\<RESULT\> actual)

    Create assertion for [`Future`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/Future.html).

    Type Parameters:`RESULT` - the type of the value contained in the [`Future`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/Future.html).Parameters:`actual` - the actual value.Returns:the created assertion object.Since:2.7.0 / 3.7.0

  * ### assertThat ###

    public static [AbstractInputStreamAssert](AbstractInputStreamAssert.html)\<?,? extends [InputStream](https://docs.oracle.com/javase/8/docs/api/java/io/InputStream.html)\> assertThat([InputStream](https://docs.oracle.com/javase/8/docs/api/java/io/InputStream.html) actual)

    Creates a new instance of `[`InputStreamAssert`](InputStreamAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractFloatAssert](AbstractFloatAssert.html)\<?\> assertThat(float actual)

    Creates a new instance of `[`FloatAssert`](FloatAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractFloatAssert](AbstractFloatAssert.html)\<?\> assertThat([Float](https://docs.oracle.com/javase/8/docs/api/java/lang/Float.html) actual)

    Creates a new instance of `[`FloatAssert`](FloatAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractFloatArrayAssert](AbstractFloatArrayAssert.html)\<?\> assertThat(float[] actual)

    Creates a new instance of `[`FloatArrayAssert`](FloatArrayAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractIntegerAssert](AbstractIntegerAssert.html)\<?\> assertThat(int actual)

    Creates a new instance of `[`IntegerAssert`](IntegerAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractIntArrayAssert](AbstractIntArrayAssert.html)\<?\> assertThat(int[] actual)

    Creates a new instance of `[`IntArrayAssert`](IntArrayAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [Int2DArrayAssert](Int2DArrayAssert.html) assertThat(int[][] actual)

    Creates a new instance of `[`Int2DArrayAssert`](Int2DArrayAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.17.0

  * ### assertThat ###

    public static [Float2DArrayAssert](Float2DArrayAssert.html) assertThat(float[][] actual)

    Creates a new instance of `[`Float2DArrayAssert`](Float2DArrayAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.17.0

  * ### assertThat ###

    public static [AbstractIntegerAssert](AbstractIntegerAssert.html)\<?\> assertThat([Integer](https://docs.oracle.com/javase/8/docs/api/java/lang/Integer.html) actual)

    Creates a new instance of `[`IntegerAssert`](IntegerAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static \<ACTUAL extends [Iterable](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html)\<? extends ELEMENT\>,ELEMENT,ELEMENT\_ASSERT extends [AbstractAssert](AbstractAssert.html)\<ELEMENT\_ASSERT,ELEMENT\>\>[FactoryBasedNavigableIterableAssert](FactoryBasedNavigableIterableAssert.html)\<?,ACTUAL,ELEMENT,ELEMENT\_ASSERT\> assertThat([Iterable](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html)\<? extends ELEMENT\> actual, [AssertFactory](AssertFactory.html)\<ELEMENT,ELEMENT\_ASSERT\> assertFactory)

    Creates a new instance of `[`FactoryBasedNavigableIterableAssert`](FactoryBasedNavigableIterableAssert.html)` allowing to navigate to any `Iterable` element in order to perform assertions on it.

     Navigational methods provided:

    * [`first()`](AbstractIterableAssert.html#first())
    * [`last()`](AbstractIterableAssert.html#last())
    * [`elements(index...)`](AbstractIterableAssert.html#elements(int...))

     The available assertions after navigating to an element depend on the `ELEMENT_ASSERT` parameter of the given [`AssertFactory<ELEMENT, ELEMENT_ASSERT>`](AssertFactory.html) (AssertJ can't figure it out because of Java type erasure).

     Example with `String` element assertions:

    ```
     Iterable<String> hobbits = newHashSet("frodo", "sam", "pippin");

     // build an AssertFactory for StringAssert (much nicer with Java 8 lambdas)
     AssertFactory<String, StringAssert> stringAssertFactory = new AssertFactory<String, StringAssert>() {
       @Override
       public StringAssert createAssert(String string) {
         return new StringAssert(string);
       }
     };

     // assertion succeeds with String assertions chained after first()
     assertThat(hobbits, stringAssertFactory).first()
                                             .startsWith("fro")
                                             .endsWith("do");
    ```

    Type Parameters:`ACTUAL` - The actual type`ELEMENT` - The actual elements type`ELEMENT_ASSERT` - The actual elements AbstractAssert typeParameters:`actual` - the actual value.`assertFactory` - the factory used to create the elements assert instance.Returns:the created assertion object.Since:2.5.0 / 3.5.0

  * ### assertThat ###

    public static \<ACTUAL extends [Iterable](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html)\<? extends ELEMENT\>,ELEMENT,ELEMENT\_ASSERT extends [AbstractAssert](AbstractAssert.html)\<ELEMENT\_ASSERT,ELEMENT\>\>[ClassBasedNavigableIterableAssert](ClassBasedNavigableIterableAssert.html)\<?,ACTUAL,ELEMENT,ELEMENT\_ASSERT\> assertThat(ACTUAL actual, [Class](https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html)\<ELEMENT\_ASSERT\> assertClass)

    Creates a new instance of `[`ClassBasedNavigableIterableAssert`](ClassBasedNavigableIterableAssert.html)` allowing to navigate to any `Iterable` element in order to perform assertions on it.

     Navigational methods provided:

    * [`first()`](AbstractIterableAssert.html#first())
    * [`last()`](AbstractIterableAssert.html#last())
    * [`element(index)`](AbstractIterableAssert.html#element(int))
    * [`element(int...)`](AbstractIterableAssert.html#elements(int...))

     The available assertions after navigating to an element depend on the given `assertClass` (AssertJ can't find the element assert type by itself because of Java type erasure).

     Example with `String` element assertions:

    ```
     Iterable<String> hobbits = newHashSet("frodo", "sam", "pippin");

     // assertion succeeds with String assertions chained after first()
     assertThat(hobbits, StringAssert.class).first()
                                            .startsWith("fro")
                                            .endsWith("do");
    ```

    Type Parameters:`ACTUAL` - The actual type`ELEMENT` - The actual elements type`ELEMENT_ASSERT` - The actual elements AbstractAssert typeParameters:`actual` - the actual value.`assertClass` - the class used to create the elements assert instance.Returns:the created assertion object.Since:2.5.0 / 3.5.0

  * ### assertThat ###

    public static \<ACTUAL extends [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)\<? extends ELEMENT\>,ELEMENT,ELEMENT\_ASSERT extends [AbstractAssert](AbstractAssert.html)\<ELEMENT\_ASSERT,ELEMENT\>\>[FactoryBasedNavigableListAssert](FactoryBasedNavigableListAssert.html)\<?,ACTUAL,ELEMENT,ELEMENT\_ASSERT\> assertThat([List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)\<? extends ELEMENT\> actual, [AssertFactory](AssertFactory.html)\<ELEMENT,ELEMENT\_ASSERT\> assertFactory)

    Creates a new instance of `[`FactoryBasedNavigableListAssert`](FactoryBasedNavigableListAssert.html)` allowing to navigate to any `List` element in order to perform assertions on it.

     Navigational methods provided:

    * [`first()`](AbstractIterableAssert.html#first())
    * [`last()`](AbstractIterableAssert.html#last())
    * [`element(index)`](AbstractIterableAssert.html#element(int))
    * [`elements(int...)`](AbstractIterableAssert.html#elements(int...))

     The available assertions after navigating to an element depend on the `ELEMENT_ASSERT` parameter of the given [`AssertFactory<ELEMENT, ELEMENT_ASSERT>`](AssertFactory.html) (AssertJ can't figure it out because of Java type erasure).

     Example with `String` element assertions:

    ```
     List<String> hobbits = newArrayList("frodo", "sam", "pippin");

     // build an AssertFactory for StringAssert (much nicer with Java 8 lambdas)
     AssertFactory<String, StringAssert> stringAssertFactory = new AssertFactory<String, StringAssert>() {
       @Override
       public StringAssert createAssert(String string) {
         return new StringAssert(string);
       }
     };

     // assertion succeeds with String assertions chained after first()
     assertThat(hobbits, stringAssertFactory).first()
                                             .startsWith("fro")
                                             .endsWith("do");
    ```

    Type Parameters:`ACTUAL` - The actual type`ELEMENT` - The actual elements type`ELEMENT_ASSERT` - The actual elements AbstractAssert typeParameters:`actual` - the actual value.`assertFactory` - the factory used to create the elements assert instance.Returns:the created assertion object.Since:2.5.0 / 3.5.0

  * ### assertThat ###

    public static \<ELEMENT,ACTUAL extends [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)\<? extends ELEMENT\>,ELEMENT\_ASSERT extends [AbstractAssert](AbstractAssert.html)\<ELEMENT\_ASSERT,ELEMENT\>\>[ClassBasedNavigableListAssert](ClassBasedNavigableListAssert.html)\<?,ACTUAL,ELEMENT,ELEMENT\_ASSERT\> assertThat([List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)\<? extends ELEMENT\> actual, [Class](https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html)\<ELEMENT\_ASSERT\> assertClass)

    Creates a new instance of `[`ClassBasedNavigableListAssert`](ClassBasedNavigableListAssert.html)` allowing to navigate to any `List` element in order to perform assertions on it.

     Navigational methods provided:

    * [`first()`](AbstractIterableAssert.html#first())
    * [`last()`](AbstractIterableAssert.html#last())
    * [`element(index)`](AbstractIterableAssert.html#element(int))
    * [`elements(int...)`](AbstractIterableAssert.html#elements(int...))

     The available assertions after navigating to an element depend on the given `assertClass` (AssertJ can't find the element assert type by itself because of Java type erasure).

     Example with `String` element assertions:

    ```
     List<String> hobbits = newArrayList("frodo", "sam", "pippin");

     // assertion succeeds with String assertions chained after first()
     assertThat(hobbits, StringAssert.class).first()
                                            .startsWith("fro")
                                            .endsWith("do");
    ```

    Type Parameters:`ELEMENT` - The actual elements type`ACTUAL` - The actual type`ELEMENT_ASSERT` - The actual elements AbstractAssert typeParameters:`actual` - the actual value.`assertClass` - the class used to create the elements assert instance.Returns:the created assertion object.Since:2.5.0 / 3.5.0

  * ### assertThat ###

    public static [AbstractLongAssert](AbstractLongAssert.html)\<?\> assertThat(long actual)

    Creates a new instance of `[`LongAssert`](LongAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractLongAssert](AbstractLongAssert.html)\<?\> assertThat([Long](https://docs.oracle.com/javase/8/docs/api/java/lang/Long.html) actual)

    Creates a new instance of `[`LongAssert`](LongAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractLongArrayAssert](AbstractLongArrayAssert.html)\<?\> assertThat(long[] actual)

    Creates a new instance of `[`LongArrayAssert`](LongArrayAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [Long2DArrayAssert](Long2DArrayAssert.html) assertThat(long[][] actual)

    Creates a new instance of `[`Long2DArrayAssert`](Long2DArrayAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.17.0

  * ### assertThat ###

    public static \<T\> [ObjectAssert](ObjectAssert.html)\<T\> assertThat(T actual)

    Creates a new instance of `[`ObjectAssert`](ObjectAssert.html)`.

    Type Parameters:`T` - the type of the actual value.Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static \<T\> [ObjectArrayAssert](ObjectArrayAssert.html)\<T\> assertThat(T[] actual)

    Creates a new instance of `[`ObjectArrayAssert`](ObjectArrayAssert.html)`.

    Type Parameters:`T` - the actual's elements type.Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static \<T\> [Object2DArrayAssert](Object2DArrayAssert.html)\<T\> assertThat(T[][] actual)

    Creates a new instance of `[`Object2DArrayAssert`](Object2DArrayAssert.html)`.

    Type Parameters:`T` - the actual's elements type.Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.17.0

  * ### assertThat ###

    public static [AbstractShortAssert](AbstractShortAssert.html)\<?\> assertThat(short actual)

    Creates a new instance of `[`ShortAssert`](ShortAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractShortAssert](AbstractShortAssert.html)\<?\> assertThat([Short](https://docs.oracle.com/javase/8/docs/api/java/lang/Short.html) actual)

    Creates a new instance of `[`ShortAssert`](ShortAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractShortArrayAssert](AbstractShortArrayAssert.html)\<?\> assertThat(short[] actual)

    Creates a new instance of `[`ShortArrayAssert`](ShortArrayAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [Short2DArrayAssert](Short2DArrayAssert.html) assertThat(short[][] actual)

    Creates a new instance of `[`Short2DArrayAssert`](Short2DArrayAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.17.0

  * ### assertThat ###

    public static [AbstractDateAssert](AbstractDateAssert.html)\<?\> assertThat([Date](https://docs.oracle.com/javase/8/docs/api/java/util/Date.html) actual)

    Creates a new instance of `[`DateAssert`](DateAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractZonedDateTimeAssert](AbstractZonedDateTimeAssert.html)\<?\> assertThat([ZonedDateTime](https://docs.oracle.com/javase/8/docs/api/java/time/ZonedDateTime.html) actual)

    Creates a new instance of `[`ZonedDateTimeAssert`](ZonedDateTimeAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractLocalDateTimeAssert](AbstractLocalDateTimeAssert.html)\<?\> assertThat([LocalDateTime](https://docs.oracle.com/javase/8/docs/api/java/time/LocalDateTime.html) actual)

    Creates a new instance of `[`LocalDateTimeAssert`](LocalDateTimeAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractOffsetDateTimeAssert](AbstractOffsetDateTimeAssert.html)\<?\> assertThat([OffsetDateTime](https://docs.oracle.com/javase/8/docs/api/java/time/OffsetDateTime.html) actual)

    Creates a new instance of `[`OffsetDateTime`](https://docs.oracle.com/javase/8/docs/api/java/time/OffsetDateTime.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractOffsetTimeAssert](AbstractOffsetTimeAssert.html)\<?\> assertThat([OffsetTime](https://docs.oracle.com/javase/8/docs/api/java/time/OffsetTime.html) actual)

    Create assertion for [`OffsetTime`](https://docs.oracle.com/javase/8/docs/api/java/time/OffsetTime.html).

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractLocalTimeAssert](AbstractLocalTimeAssert.html)\<?\> assertThat([LocalTime](https://docs.oracle.com/javase/8/docs/api/java/time/LocalTime.html) actual)

    Creates a new instance of `[`LocalTimeAssert`](LocalTimeAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractLocalDateAssert](AbstractLocalDateAssert.html)\<?\> assertThat([LocalDate](https://docs.oracle.com/javase/8/docs/api/java/time/LocalDate.html) actual)

    Creates a new instance of `[`LocalDateAssert`](LocalDateAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static [AbstractInstantAssert](AbstractInstantAssert.html)\<?\> assertThat([Instant](https://docs.oracle.com/javase/8/docs/api/java/time/Instant.html) actual)

    Creates a new instance of `[`InstantAssert`](InstantAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.7.0

  * ### assertThat ###

    public static [AbstractDurationAssert](AbstractDurationAssert.html)\<?\> assertThat([Duration](https://docs.oracle.com/javase/8/docs/api/java/time/Duration.html) actual)

    Creates a new instance of `[`DurationAssert`](DurationAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.15.0

  * ### assertThat ###

    public static [AbstractPeriodAssert](AbstractPeriodAssert.html)\<?\> assertThat([Period](https://docs.oracle.com/javase/8/docs/api/java/time/Period.html) actual)

    Creates a new instance of `[`PeriodAssert`](PeriodAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.17.0

  * ### assertThat ###

    public static [AtomicBooleanAssert](AtomicBooleanAssert.html) assertThat([AtomicBoolean](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicBoolean.html) actual)

    Create assertion for [`AtomicBoolean`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicBoolean.html).

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:2.7.0 / 3.7.0

  * ### assertThat ###

    public static [AtomicIntegerAssert](AtomicIntegerAssert.html) assertThat([AtomicInteger](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicInteger.html) actual)

    Create assertion for [`AtomicInteger`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicInteger.html).

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:2.7.0 / 3.7.0

  * ### assertThat ###

    public static [AtomicIntegerArrayAssert](AtomicIntegerArrayAssert.html) assertThat([AtomicIntegerArray](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicIntegerArray.html) actual)

    Create int[] assertion for [`AtomicIntegerArray`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicIntegerArray.html).

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:2.7.0 / 3.7.0

  * ### assertThat ###

    public static \<OBJECT\>[AtomicIntegerFieldUpdaterAssert](AtomicIntegerFieldUpdaterAssert.html)\<OBJECT\> assertThat([AtomicIntegerFieldUpdater](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicIntegerFieldUpdater.html)\<OBJECT\> actual)

    Create assertion for [`AtomicIntegerFieldUpdater`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicIntegerFieldUpdater.html).

    Type Parameters:`OBJECT` - the type of the object holding the updatable field.Parameters:`actual` - the actual value.Returns:the created assertion object.Since:2.7.0 / 3.7.0

  * ### assertThat ###

    public static [LongAdderAssert](LongAdderAssert.html) assertThat([LongAdder](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/LongAdder.html) actual)

    Create assertion for [`LongAdder`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/LongAdder.html).

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.16.0

  * ### assertThat ###

    public static [AtomicLongAssert](AtomicLongAssert.html) assertThat([AtomicLong](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicLong.html) actual)

    Create assertion for [`AtomicLong`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicLong.html).

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:2.7.0 / 3.7.0

  * ### assertThat ###

    public static [AtomicLongArrayAssert](AtomicLongArrayAssert.html) assertThat([AtomicLongArray](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicLongArray.html) actual)

    Create assertion for [`AtomicLongArray`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicLongArray.html).

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:2.7.0 / 3.7.0

  * ### assertThat ###

    public static \<OBJECT\>[AtomicLongFieldUpdaterAssert](AtomicLongFieldUpdaterAssert.html)\<OBJECT\> assertThat([AtomicLongFieldUpdater](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicLongFieldUpdater.html)\<OBJECT\> actual)

    Create assertion for [`AtomicLongFieldUpdater`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicLongFieldUpdater.html).

    Type Parameters:`OBJECT` - the type of the object holding the updatable field.Parameters:`actual` - the actual value.Returns:the created assertion object.Since:2.7.0 / 3.7.0

  * ### assertThat ###

    public static \<VALUE\> [AtomicReferenceAssert](AtomicReferenceAssert.html)\<VALUE\> assertThat([AtomicReference](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicReference.html)\<VALUE\> actual)

    Create assertion for [`AtomicReference`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicReference.html).

    Type Parameters:`VALUE` - the type of the value contained in the [`AtomicReference`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicReference.html).Parameters:`actual` - the actual value.Returns:the created assertion object.Since:2.7.0 / 3.7.0

  * ### assertThat ###

    public static \<ELEMENT\>[AtomicReferenceArrayAssert](AtomicReferenceArrayAssert.html)\<ELEMENT\> assertThat([AtomicReferenceArray](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicReferenceArray.html)\<ELEMENT\> actual)

    Create assertion for [`AtomicReferenceArray`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicReferenceArray.html).

    Type Parameters:`ELEMENT` - the type of the value contained in the [`AtomicReferenceArray`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicReferenceArray.html).Parameters:`actual` - the actual value.Returns:the created assertion object.Since:2.7.0 / 3.7.0

  * ### assertThat ###

    public static \<FIELD,OBJECT\>[AtomicReferenceFieldUpdaterAssert](AtomicReferenceFieldUpdaterAssert.html)\<FIELD,OBJECT\> assertThat([AtomicReferenceFieldUpdater](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicReferenceFieldUpdater.html)\<OBJECT,FIELD\> actual)

    Create assertion for [`AtomicReferenceFieldUpdater`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicReferenceFieldUpdater.html).

    Type Parameters:`FIELD` - the type of the field which gets updated by the [`AtomicReferenceFieldUpdater`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicReferenceFieldUpdater.html).`OBJECT` - the type of the object holding the updatable field.Parameters:`actual` - the actual value.Returns:the created assertion object.Since:2.7.0 / 3.7.0

  * ### assertThat ###

    public static \<VALUE\>[AtomicMarkableReferenceAssert](AtomicMarkableReferenceAssert.html)\<VALUE\> assertThat([AtomicMarkableReference](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicMarkableReference.html)\<VALUE\> actual)

    Create assertion for [`AtomicMarkableReference`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicMarkableReference.html).

    Type Parameters:`VALUE` - the type of the value contained in the [`AtomicMarkableReference`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicMarkableReference.html).Parameters:`actual` - the actual value.Returns:the created assertion object.Since:2.7.0 / 3.7.0

  * ### assertThat ###

    public static \<VALUE\>[AtomicStampedReferenceAssert](AtomicStampedReferenceAssert.html)\<VALUE\> assertThat([AtomicStampedReference](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicStampedReference.html)\<VALUE\> actual)

    Create assertion for [`AtomicStampedReference`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicStampedReference.html).

    Type Parameters:`VALUE` - the type of the value contained in the [`AtomicStampedReference`](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicStampedReference.html).Parameters:`actual` - the actual value.Returns:the created assertion object.Since:2.7.0 / 3.7.0

  * ### assertThat ###

    public static \<T extends [Throwable](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html)\>[AbstractThrowableAssert](AbstractThrowableAssert.html)\<?,T\> assertThat(T actual)

    Creates a new instance of `[`ThrowableAssert`](ThrowableAssert.html)`.

    Type Parameters:`T` - the type of the actual throwable.Parameters:`actual` - the actual value.Returns:the created [`ThrowableAssert`](ThrowableAssert.html).

  * ### assertThatThrownBy ###

    public static [AbstractThrowableAssert](AbstractThrowableAssert.html)\<?,? extends [Throwable](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html)\> assertThatThrownBy([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseThrowable)

    Allows to capture and then assert on a [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) (easier done with lambdas).

     Java 8 example :

    ```
     @Test
      public void testException() {
        assertThatThrownBy(() -> { throw new Exception("boom!"); }).isInstanceOf(Exception.class)
                                                                   .hasMessageContaining("boom");
     }
    ```

     If the provided [`ThrowableAssert.ThrowingCallable`](ThrowableAssert.ThrowingCallable.html) does not raise an exception, an error is immediately thrown, in that case the test description provided with [`as(String, Object...)`](Descriptable.html#as(java.lang.String,java.lang.Object...)) is not honored.  
     To use a test description, use [`catchThrowable(ThrowingCallable)`](#catchThrowable(org.assertj.core.api.ThrowableAssert.ThrowingCallable)) as shown below:

    ```
     // assertion will fail but "display me" won't appear in the error
     assertThatThrownBy(() -> {}).as("display me")
                                 .isInstanceOf(Exception.class);

     // assertion will fail AND "display me" will appear in the error
     Throwable thrown = catchThrowable(() -> {});
     assertThat(thrown).as("display me")
                       .isInstanceOf(Exception.class);
    ```

     Alternatively you can also use `assertThatCode(ThrowingCallable)` for the test description provided with [`as(String, Object...)`](Descriptable.html#as(java.lang.String,java.lang.Object...)) to always be honored.

    Parameters:`shouldRaiseThrowable` - The [`ThrowableAssert.ThrowingCallable`](ThrowableAssert.ThrowingCallable.html) or lambda with the code that should raise the throwable.Returns:the created [`ThrowableAssert`](ThrowableAssert.html).

  * ### assertThatThrownBy ###

    public static [AbstractThrowableAssert](AbstractThrowableAssert.html)\<?,? extends [Throwable](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html)\> assertThatThrownBy([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseThrowable, [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) description, [Object](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html)... args)

    Allows to capture and then assert on a [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) like `assertThatThrownBy(ThrowingCallable)` but this method let you set the assertion description the same way you do with [`as(String, Object...)`](Descriptable.html#as(java.lang.String,java.lang.Object...)).

     Example:

    ```
     @Test
      public void testException() {
        // if this assertion failed (but it doesn't), the error message would start with [Test explosive code]
        assertThatThrownBy(() -> { throw new IOException("boom!"); }, "Test explosive code")
                 .isInstanceOf(IOException.class)
                 .hasMessageContaining("boom");
     }
    ```

     If the provided [`ThrowingCallable`](ThrowableAssert.ThrowingCallable.html) does not raise an exception, an error is immediately thrown.

     The test description provided is honored but not the one with [`as(String, Object...)`](Descriptable.html#as(java.lang.String,java.lang.Object...)), example:

    ```
     // assertion will fail but "display me" won't appear in the error message
     assertThatThrownBy(() -> {}).as("display me")
                                 .isInstanceOf(Exception.class);

     // assertion will fail AND "display me" will appear in the error message
     assertThatThrownBy(() -> {}, "display me")
                                 .isInstanceOf(Exception.class);
    ```

    Parameters:`shouldRaiseThrowable` - The [`ThrowableAssert.ThrowingCallable`](ThrowableAssert.ThrowingCallable.html) or lambda with the code that should raise the throwable.`description` - the new description to set.`args` - optional parameter if description is a format String.Returns:the created [`ThrowableAssert`](ThrowableAssert.html).Since:3.9.0

  * ### assertThatCode ###

    public static [AbstractThrowableAssert](AbstractThrowableAssert.html)\<?,? extends [Throwable](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html)\> assertThatCode([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseOrNotThrowable)

    Allows to capture and then assert on a [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) (easier done with lambdas).

     The main difference with `assertThatThrownBy(ThrowingCallable)` is that this method does not fail if no exception was thrown.

     Example :

    ```
     ThrowingCallable boomCode = () -> {
       throw new Exception("boom!");
     };
     ThrowingCallable doNothing = () -> {};

     // assertions succeed
     assertThatCode(doNothing).doesNotThrowAnyException();
     assertThatCode(boomCode).isInstanceOf(Exception.class)
                             .hasMessageContaining("boom");

     // assertion fails
     assertThatCode(boomCode).doesNotThrowAnyException();
    ```

     Contrary to `assertThatThrownBy(ThrowingCallable)` the test description provided with [`as(String, Object...)`](Descriptable.html#as(java.lang.String,java.lang.Object...)) is always honored as shown below.

    ```
     ThrowingCallable doNothing = () -> {};

     // assertion fails and "display me" appears in the assertion error
     assertThatCode(doNothing).as("display me")
                              .isInstanceOf(Exception.class);
    ```

     This method was not named `assertThat` because the java compiler reported it ambiguous when used directly with a lambda :(

    Parameters:`shouldRaiseOrNotThrowable` - The [`ThrowableAssert.ThrowingCallable`](ThrowableAssert.ThrowingCallable.html) or lambda with the code that should raise the throwable.Returns:the created [`ThrowableAssert`](ThrowableAssert.html).Since:3.7.0

  * ### assertThatObject ###

    public static \<T\> [ObjectAssert](ObjectAssert.html)\<T\> assertThatObject(T actual)

    Creates a new instance of `[`ObjectAssert`](ObjectAssert.html)` for any object.

     This overload is useful, when an overloaded method of assertThat(...) takes precedence over the generic [`assertThat(Object)`](#assertThat(T)).

     Example:

     Cast necessary because [`assertThat(List)`](#assertThat(java.util.List)) "forgets" actual type:

    ```
    assertThat(new LinkedList<>(asList("abc"))).matches(list -> ((Deque<String>) list).getFirst().equals("abc"));
    ```

     No cast needed, but also no additional list assertions:

    ```
    assertThatObject(new LinkedList<>(asList("abc"))).matches(list -> list.getFirst().equals("abc"));
    ```

    Type Parameters:`T` - the type of the actual value.Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.12.0

  * ### assertWith ###

    [@SafeVarargs](https://docs.oracle.com/javase/8/docs/api/java/lang/SafeVarargs.html)public static \<T\> [ObjectAssert](ObjectAssert.html)\<T\> assertWith(T actual, [Consumer](https://docs.oracle.com/javase/8/docs/api/java/util/function/Consumer.html)\<T\>... requirements)

    Uses the given instance as the instance under test for all the assertions expressed as the passed [`Consumer`](https://docs.oracle.com/javase/8/docs/api/java/util/function/Consumer.html).

     This is useful to avoid repeating getting the instance to test, a bit like a [with](https://mrhaki.blogspot.com/2009/09/groovy-goodness-with-method.html) block which turns the target into the equivalent of `this` (as in Groovy for example).

     Example:

    ```
     assertWith(team.getPlayers().get(0).getStats(),
                stats -> {
                   assertThat(stats.pointPerGame).isGreaterThan(25.7);
                   assertThat(stats.assistsPerGame).isGreaterThan(7.2);
                   assertThat(stats.reboundsPerGame).isBetween(9, 12);
                });
    ```

    `assertWith` is variation of [`AbstractAssert.satisfies(Consumer[])`](AbstractAssert.html#satisfies(java.util.function.Consumer...)) hopefully easier to find for some users.

    Type Parameters:`T` - the type of the actual value.Parameters:`actual` - the actual value.`requirements` - to assert on the actual object - must not be null.Returns:the created assertion object.Since:3.20.0

  * ### catchThrowable ###

    public static [Throwable](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) catchThrowable([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseThrowable)

    Allows catching a [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) more easily when used with Java 8 lambdas.

     This caught [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) can then be asserted.

     If you need to assert on the real type of Throwable caught (e.g. IOException), use [`catchThrowableOfType(ThrowingCallable, Class)`](#catchThrowableOfType(org.assertj.core.api.ThrowableAssert.ThrowingCallable,java.lang.Class)).

     Example:

    ```
    @Test
     public void testException() {
       // when
       Throwable thrown = catchThrowable(() -> { throw new Exception("boom!"); });

       // then
       assertThat(thrown).isInstanceOf(Exception.class)
                         .hasMessageContaining("boom");
     }
    ```

    Parameters:`shouldRaiseThrowable` - The lambda with the code that should raise the exception.Returns:The captured exception or `null` if none was raised by the callable.See Also:
    * [`catchThrowableOfType(ThrowingCallable, Class)`](#catchThrowableOfType(org.assertj.core.api.ThrowableAssert.ThrowingCallable,java.lang.Class))

  * ### catchThrowableOfType ###

    public static \<THROWABLE extends [Throwable](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html)\>THROWABLE catchThrowableOfType([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseThrowable, [Class](https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html)\<THROWABLE\> type)

    Allows catching a [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) of a specific type.

     A call is made to `catchThrowable(ThrowingCallable)`, if no exception is thrown it returns null otherwise it checks that the caught [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) has the specified type and casts it making it convenient to perform subtype-specific assertions on it.

     Example:

    ```
     class TextException extends Exception {
       int line;
       int column;

       public TextException(String msg, int line, int column) {
         super(msg);
         this.line = line;
         this.column = column;
       }
     }

     TextException textException = catchThrowableOfType(() -> { throw new TextException("boom!", 1, 5); },
                                                        TextException.class);
     // assertions succeed
     assertThat(textException).hasMessage("boom!");
     assertThat(textException.line).isEqualTo(1);
     assertThat(textException.column).isEqualTo(5);

     // succeeds as catchThrowableOfType returns null when the code does not thrown any exceptions
     assertThat(catchThrowableOfType(() -> {}, Exception.class)).isNull();

     // fails as TextException is not a RuntimeException
     catchThrowableOfType(() -> { throw new TextException("boom!", 1, 5); }, RuntimeException.class);
    ```

    Type Parameters:`THROWABLE` - the [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) type.Parameters:`shouldRaiseThrowable` - The lambda with the code that should raise the exception.`type` - The type of exception that the code is expected to raise.Returns:The captured exception or `null` if none was raised by the callable.Since:3.9.0See Also:
    * [`catchThrowable(ThrowingCallable)`](#catchThrowable(org.assertj.core.api.ThrowableAssert.ThrowingCallable))

  * ### catchException ###

    public static [Exception](https://docs.oracle.com/javase/8/docs/api/java/lang/Exception.html) catchException([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseException)

    Allows catching an instance of [`Exception`](https://docs.oracle.com/javase/8/docs/api/java/lang/Exception.html).

     A call is made to `catchThrowable(ThrowingCallable)`, if no exception is thrown it returns null otherwise it checks that the caught [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) is of type [`Exception`](https://docs.oracle.com/javase/8/docs/api/java/lang/Exception.html) and casts it making it convenient to perform subtype-specific assertions on it.

     Example:

    ```

     Exception exception = catchException(() -> {throw new Exception("boom!");});
     // assertions succeed
     assertThat(exception).hasMessage("boom!");

     // succeeds as catchException returns null when the code does not throw any exceptions
     assertThat(catchException(() -> {})).isNull();

     // fails as the thrown instance is not an Exception
     catchException(() -> {throw new Throwable("boom!");});
    ```

    Parameters:`shouldRaiseException` - The lambda with the code that should raise the exception.Returns:The captured exception or `null` if none was raised by the callable.Since:3.22.0See Also:
    * [`catchThrowable(ThrowingCallable)`](#catchThrowable(org.assertj.core.api.ThrowableAssert.ThrowingCallable))

  * ### catchRuntimeException ###

    public static [RuntimeException](https://docs.oracle.com/javase/8/docs/api/java/lang/RuntimeException.html) catchRuntimeException([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseRuntimeException)

    Allows catching an instance of [`RuntimeException`](https://docs.oracle.com/javase/8/docs/api/java/lang/RuntimeException.html).

     A call is made to `catchThrowable(ThrowingCallable)`, if no exception is thrown it returns null otherwise it checks that the caught [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) is of type [`RuntimeException`](https://docs.oracle.com/javase/8/docs/api/java/lang/RuntimeException.html) and casts it making it convenient to perform subtype-specific assertions on it.

     Example:

    ```

     RuntimeException runtimeException = catchRuntimeException(() -> {throw new RuntimeException("boom!");});
     // assertions succeed
     assertThat(runtimeException).hasMessage("boom!");

     // succeeds as catchRuntimeException returns null when the code does not throw any exceptions
     assertThat(catchRuntimeException(() -> {})).isNull();

     // fails as the thrown instance is not a RuntimeException
     catchRuntimeException(() -> {throw new Exception("boom!");});
    ```

    Parameters:`shouldRaiseRuntimeException` - The lambda with the code that should raise the exception.Returns:The captured exception or `null` if none was raised by the callable.Since:3.22.0See Also:
    * [`catchThrowable(ThrowingCallable)`](#catchThrowable(org.assertj.core.api.ThrowableAssert.ThrowingCallable))

  * ### catchNullPointerException ###

    public static [NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html) catchNullPointerException([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseNullPointerException)

    Allows catching an instance of [`NullPointerException`](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html).

     A call is made to `catchThrowable(ThrowingCallable)`, if no exception is thrown it returns null otherwise it checks that the caught [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) is of type [`RuntimeException`](https://docs.oracle.com/javase/8/docs/api/java/lang/RuntimeException.html) and casts it making it convenient to perform subtype-specific assertions on it.

     Example:

    ```

     NullPointerException nullPointerException = catchNullPointerException(() -> {throw new NullPointerException("boom!");});
     // assertions succeed
     assertThat(nullPointerException).hasMessage("boom!");

     // succeeds as catchNullPointerException returns null when the code does not throw any exceptions
     assertThat(catchNullPointerException(() -> {})).isNull();

     // fails as the thrown instance is not a NullPointerException
     catchNullPointerException(() -> {throw new Exception("boom!");});
    ```

    Parameters:`shouldRaiseNullPointerException` - The lambda with the code that should raise the exception.Returns:The captured exception or `null` if none was raised by the callable.Since:3.22.0See Also:
    * [`catchThrowable(ThrowingCallable)`](#catchThrowable(org.assertj.core.api.ThrowableAssert.ThrowingCallable))

  * ### catchIllegalArgumentException ###

    public static [IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html) catchIllegalArgumentException([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseIllegalArgumentException)

    Allows catching an instance of [`IllegalArgumentException`](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html).

     A call is made to `catchThrowable(ThrowingCallable)`, if no exception is thrown it returns null otherwise it checks that the caught [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) is of type [`IllegalArgumentException`](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html) and casts it making it convenient to perform subtype-specific assertions on it.

     Example:

    ```

     IllegalArgumentException illegalArgumentException = catchIllegalArgumentException(() -> {throw new IllegalArgumentException("boom!");});
     // assertions succeed
     assertThat(illegalArgumentException).hasMessage("boom!");

     // succeeds as catchNullPointerException returns null when the code does not throw any exceptions
     assertThat(catchIllegalArgumentException(() -> {})).isNull();

     // fails as the thrown instance is not an IllegalArgumentException
     catchIllegalArgumentException(() -> {throw new Exception("boom!");});
    ```

    Parameters:`shouldRaiseIllegalArgumentException` - The lambda with the code that should raise the exception.Returns:The captured exception or `null` if none was raised by the callable.Since:3.22.0See Also:
    * [`catchThrowable(ThrowingCallable)`](#catchThrowable(org.assertj.core.api.ThrowableAssert.ThrowingCallable))

  * ### catchIOException ###

    public static [IOException](https://docs.oracle.com/javase/8/docs/api/java/io/IOException.html) catchIOException([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseIOException)

    Allows catching an instance of [`IOException`](https://docs.oracle.com/javase/8/docs/api/java/io/IOException.html).

     A call is made to `catchThrowable(ThrowingCallable)`, if no exception is thrown it returns null otherwise it checks that the caught [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) is of type [`IOException`](https://docs.oracle.com/javase/8/docs/api/java/io/IOException.html) and casts it making it convenient to perform subtype-specific assertions on it.

     Example:

    ```

     IOException iOException = catchIOException(() -> {throw new IOException("boom!");});
     // assertions succeed
     assertThat(iOException).hasMessage("boom!");

     // succeeds as catchIOException returns null when the code does not throw any exceptions
     assertThat(catchIOException(() -> {})).isNull();

     // fails as the thrown instance is not an IOException
     catchIOException(() -> {throw new Exception("boom!");});
    ```

    Parameters:`shouldRaiseIOException` - The lambda with the code that should raise the exception.Returns:The captured exception or `null` if none was raised by the callable.Since:3.22.0See Also:
    * [`catchThrowable(ThrowingCallable)`](#catchThrowable(org.assertj.core.api.ThrowableAssert.ThrowingCallable))

  * ### catchReflectiveOperationException ###

    public static [ReflectiveOperationException](https://docs.oracle.com/javase/8/docs/api/java/lang/ReflectiveOperationException.html) catchReflectiveOperationException([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseReflectiveOperationException)

    Allows catching an instance of [`ReflectiveOperationException`](https://docs.oracle.com/javase/8/docs/api/java/lang/ReflectiveOperationException.html).

     A call is made to `catchThrowable(ThrowingCallable)`, if no exception is thrown it returns null otherwise it checks that the caught [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) is of type [`ReflectiveOperationException`](https://docs.oracle.com/javase/8/docs/api/java/lang/ReflectiveOperationException.html) and casts it making it convenient to perform subtype-specific assertions on it.

     Example:

    ```

     ReflectiveOperationException reflectiveOperationException = catchReflectiveOperationException(() -> {throw new ReflectiveOperationException("boom!");});
     // assertions succeed
     assertThat(reflectiveOperationException).hasMessage("boom!");

     // succeeds as catchReflectiveOperationException returns null when the code does not throw any exceptions
     assertThat(catchReflectiveOperationException(() -> {})).isNull();

     // fails as the thrown instance is not an IOException
     catchReflectiveOperationException(() -> {throw new Exception("boom!");});
    ```

    Parameters:`shouldRaiseReflectiveOperationException` - The lambda with the code that should raise the exception.Returns:The captured exception or `null` if none was raised by the callable.Since:3.22.0See Also:
    * [`catchThrowable(ThrowingCallable)`](#catchThrowable(org.assertj.core.api.ThrowableAssert.ThrowingCallable))

  * ### catchIllegalStateException ###

    public static [IllegalStateException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalStateException.html) catchIllegalStateException([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseIllegalStateException)

    Allows catching an instance of [`IllegalStateException`](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalStateException.html).

     A call is made to `catchThrowable(ThrowingCallable)`, if no exception is thrown it returns null otherwise it checks that the caught [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) is of type [`IllegalStateException`](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalStateException.html) and casts it making it convenient to perform subtype-specific assertions on it.

     Example:

    ```

     IllegalStateException illegalStateException = catchIllegalStateException(() -> {throw new IllegalStateException("boom!");});
     // assertions succeed
     assertThat(illegalStateException).hasMessage("boom!");

     // succeeds as catchReflectiveOperationException returns null when the code does not throw any exceptions
     assertThat(catchIllegalStateException(() -> {})).isNull();

     // fails as the thrown instance is not an IOException
     catchIllegalStateException(() -> {throw new Exception("boom!");});
    ```

    Parameters:`shouldRaiseIllegalStateException` - The lambda with the code that should raise the exception.Returns:The captured exception or `null` if none was raised by the callable.Since:3.22.0See Also:
    * [`catchThrowable(ThrowingCallable)`](#catchThrowable(org.assertj.core.api.ThrowableAssert.ThrowingCallable))

  * ### catchIndexOutOfBoundsException ###

    public static [IndexOutOfBoundsException](https://docs.oracle.com/javase/8/docs/api/java/lang/IndexOutOfBoundsException.html) catchIndexOutOfBoundsException([ThrowableAssert.ThrowingCallable](ThrowableAssert.ThrowingCallable.html) shouldRaiseIndexOutOfBoundException)

    Allows catching an instance of [`IndexOutOfBoundsException`](https://docs.oracle.com/javase/8/docs/api/java/lang/IndexOutOfBoundsException.html).

     A call is made to `catchThrowable(ThrowingCallable)`, if no exception is thrown it returns null otherwise it checks that the caught [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) is of type [`IndexOutOfBoundsException`](https://docs.oracle.com/javase/8/docs/api/java/lang/IndexOutOfBoundsException.html) and casts it making it convenient to perform subtype-specific assertions on it.

     Example:

    ```

     IndexOutOfBoundsException indexOutOfBoundsException = catchIndexOutOfBoundsException(() -> {throw new IndexOutOfBoundsException("boom!");});
     // assertions succeed
     assertThat(indexOutOfBoundsException).hasMessage("boom!");

     // succeeds as catchIndexOutOfBoundsException returns null when the code does not throw any exceptions
     assertThat(catchIndexOutOfBoundsException(() -> {})).isNull();

     // fails as the thrown instance is not an IOException
     catchIndexOutOfBoundsException(() -> {throw new Exception("boom!");});
    ```

    Parameters:`shouldRaiseIndexOutOfBoundException` - The lambda with the code that should raise the exception.Returns:The captured exception or `null` if none was raised by the callable.Since:3.22.0See Also:
    * [`catchThrowable(ThrowingCallable)`](#catchThrowable(org.assertj.core.api.ThrowableAssert.ThrowingCallable))

  * ### assertThatExceptionOfType ###

    public static \<T extends [Throwable](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html)\>[ThrowableTypeAssert](ThrowableTypeAssert.html)\<T\> assertThatExceptionOfType([Class](https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html)\<? extends T\> exceptionType)

    Entry point to check that an exception of type T is thrown by a given `throwingCallable` which allows to chain assertions on the thrown exception.

     Example:

    ```
     assertThatExceptionOfType(IOException.class)
               .isThrownBy(() -> { throw new IOException("boom!"); })
               .withMessage("boom!");
    ```

     This method is more or less the same of [`assertThatThrownBy(ThrowingCallable)`](#assertThatThrownBy(org.assertj.core.api.ThrowableAssert.ThrowingCallable)) but in a more natural way.

    Type Parameters:`T` - the exception type.Parameters:`exceptionType` - the exception type class.Returns:the created [`ThrowableTypeAssert`](ThrowableTypeAssert.html).

  * ### assertThatNoException ###

    public static [NotThrownAssert](NotThrownAssert.html) assertThatNoException()

    Entry point to check that no exception of any type is thrown by a given `throwingCallable`.

     Example:

    ```
    assertThatNoException().isThrownBy(() -> System.out.println("OK"));
    ```

     This method is more or less the same of `assertThatCode(...).doesNotThrowAnyException();` but in a more natural way.

    Returns:the created [`NotThrownAssert`](NotThrownAssert.html).Since:3.17.0

  * ### assertThatNullPointerException ###

    public static [ThrowableTypeAssert](ThrowableTypeAssert.html)\<[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)\> assertThatNullPointerException()

    Alias for [`assertThatExceptionOfType(Class)`](#assertThatExceptionOfType(java.lang.Class)) for [`NullPointerException`](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html).

    Returns:the created [`ThrowableTypeAssert`](ThrowableTypeAssert.html).Since:3.7.0

  * ### assertThatIllegalArgumentException ###

    public static [ThrowableTypeAssert](ThrowableTypeAssert.html)\<[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)\> assertThatIllegalArgumentException()

    Alias for [`assertThatExceptionOfType(Class)`](#assertThatExceptionOfType(java.lang.Class)) for [`IllegalArgumentException`](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html).

    Returns:the created [`ThrowableTypeAssert`](ThrowableTypeAssert.html).Since:3.7.0

  * ### assertThatIOException ###

    public static [ThrowableTypeAssert](ThrowableTypeAssert.html)\<[IOException](https://docs.oracle.com/javase/8/docs/api/java/io/IOException.html)\> assertThatIOException()

    Alias for [`assertThatExceptionOfType(Class)`](#assertThatExceptionOfType(java.lang.Class)) for [`IOException`](https://docs.oracle.com/javase/8/docs/api/java/io/IOException.html).

    Returns:the created [`ThrowableTypeAssert`](ThrowableTypeAssert.html).Since:3.7.0

  * ### assertThatIllegalStateException ###

    public static [ThrowableTypeAssert](ThrowableTypeAssert.html)\<[IllegalStateException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalStateException.html)\> assertThatIllegalStateException()

    Alias for [`assertThatExceptionOfType(Class)`](#assertThatExceptionOfType(java.lang.Class)) for [`IllegalStateException`](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalStateException.html).

    Returns:the created [`ThrowableTypeAssert`](ThrowableTypeAssert.html).Since:3.7.0

  * ### assertThatException ###

    public static [ThrowableTypeAssert](ThrowableTypeAssert.html)\<[Exception](https://docs.oracle.com/javase/8/docs/api/java/lang/Exception.html)\> assertThatException()

    Alias for [`assertThatExceptionOfType(Class)`](#assertThatExceptionOfType(java.lang.Class)) for [`Exception`](https://docs.oracle.com/javase/8/docs/api/java/lang/Exception.html).

    Returns:the created [`ThrowableTypeAssert`](ThrowableTypeAssert.html).Since:3.23.0

  * ### assertThatRuntimeException ###

    public static [ThrowableTypeAssert](ThrowableTypeAssert.html)\<[RuntimeException](https://docs.oracle.com/javase/8/docs/api/java/lang/RuntimeException.html)\> assertThatRuntimeException()

    Alias for [`assertThatExceptionOfType(Class)`](#assertThatExceptionOfType(java.lang.Class)) for [`RuntimeException`](https://docs.oracle.com/javase/8/docs/api/java/lang/RuntimeException.html).

    Returns:the created [`ThrowableTypeAssert`](ThrowableTypeAssert.html).Since:3.23.0

  * ### assertThatReflectiveOperationException ###

    public static [ThrowableTypeAssert](ThrowableTypeAssert.html)\<[ReflectiveOperationException](https://docs.oracle.com/javase/8/docs/api/java/lang/ReflectiveOperationException.html)\> assertThatReflectiveOperationException()

    Alias for [`assertThatExceptionOfType(Class)`](#assertThatExceptionOfType(java.lang.Class)) for [`ReflectiveOperationException`](https://docs.oracle.com/javase/8/docs/api/java/lang/ReflectiveOperationException.html).

    Returns:the created [`ThrowableTypeAssert`](ThrowableTypeAssert.html).Since:3.23.0

  * ### assertThatIndexOutOfBoundsException ###

    public static [ThrowableTypeAssert](ThrowableTypeAssert.html)\<[IndexOutOfBoundsException](https://docs.oracle.com/javase/8/docs/api/java/lang/IndexOutOfBoundsException.html)\> assertThatIndexOutOfBoundsException()

    Alias for [`assertThatExceptionOfType(Class)`](#assertThatExceptionOfType(java.lang.Class)) for [`IndexOutOfBoundsException`](https://docs.oracle.com/javase/8/docs/api/java/lang/IndexOutOfBoundsException.html).

    Returns:the created [`ThrowableTypeAssert`](ThrowableTypeAssert.html).Since:3.23.0

  * ### setRemoveAssertJRelatedElementsFromStackTrace ###

    public static void setRemoveAssertJRelatedElementsFromStackTrace(boolean removeAssertJRelatedElementsFromStackTrace)

    Sets whether we remove elements related to AssertJ from assertion error stack trace.

     Default is [true](../configuration/Configuration.html#REMOVE_ASSERTJ_RELATED_ELEMENTS_FROM_STACK_TRACE).

    Parameters:`removeAssertJRelatedElementsFromStackTrace` - flag.

  * ### fail ###

    public static \<T\> T fail([String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) failureMessage)

    Throws an [`AssertionError`](https://docs.oracle.com/javase/8/docs/api/java/lang/AssertionError.html) with the given message.

    Type Parameters:`T` - dummy return value typeParameters:`failureMessage` - error message.Returns:nothing, it's just to be used in `doSomething(optional.orElseGet(() -> fail("boom")));`.Throws:`[AssertionError](https://docs.oracle.com/javase/8/docs/api/java/lang/AssertionError.html)` - with the given message.

  * ### fail ###

    public static \<T\> T fail([String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) failureMessage, [Object](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html)... args)

    Throws an [`AssertionError`](https://docs.oracle.com/javase/8/docs/api/java/lang/AssertionError.html) with the given message built as [`String.format(String, Object...)`](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html#format-java.lang.String-java.lang.Object...-).

    Type Parameters:`T` - dummy return value typeParameters:`failureMessage` - error message.`args` - Arguments referenced by the format specifiers in the format string.Returns:nothing, it's just to be used in `doSomething(optional.orElseGet(() -> fail("b%s", "oom")));`.Throws:`[AssertionError](https://docs.oracle.com/javase/8/docs/api/java/lang/AssertionError.html)` - with the given built message.

  * ### fail ###

    public static \<T\> T fail([String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) failureMessage, [Throwable](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) realCause)

    Throws an [`AssertionError`](https://docs.oracle.com/javase/8/docs/api/java/lang/AssertionError.html) with the given message and with the [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) that caused the failure.

    Type Parameters:`T` - dummy return value typeParameters:`failureMessage` - the description of the failed assertion. It can be `null`.`realCause` - cause of the error.Returns:nothing, it's just to be used in `doSomething(optional.orElseGet(() -> fail("boom", cause)));`.Throws:`[AssertionError](https://docs.oracle.com/javase/8/docs/api/java/lang/AssertionError.html)` - with the given message and with the [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) that caused the failure.

  * ### failBecauseExceptionWasNotThrown ###

    public static \<T\> T failBecauseExceptionWasNotThrown([Class](https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html)\<? extends [Throwable](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html)\> throwableClass)

    Throws an [`AssertionError`](https://docs.oracle.com/javase/8/docs/api/java/lang/AssertionError.html) with a message explaining that a [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) of given class was expected to be thrown but had not been.

    [`shouldHaveThrown(Class)`](#shouldHaveThrown(java.lang.Class)) can be used as a replacement.

    Type Parameters:`T` - dummy return value typeParameters:`throwableClass` - the Throwable class that was expected to be thrown.Returns:nothing, it's just to be used in `doSomething(optional.orElseGet(() -> failBecauseExceptionWasNotThrown(IOException.class)));`.Throws:`[AssertionError](https://docs.oracle.com/javase/8/docs/api/java/lang/AssertionError.html)` - with a message explaining that a [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) of given class was expected to be thrown but had not been.

  * ### shouldHaveThrown ###

    public static \<T\> T shouldHaveThrown([Class](https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html)\<? extends [Throwable](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html)\> throwableClass)

    Throws an [`AssertionError`](https://docs.oracle.com/javase/8/docs/api/java/lang/AssertionError.html) with a message explaining that a [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) of given class was expected to be thrown but had not been.

    Type Parameters:`T` - dummy return value typeParameters:`throwableClass` - the Throwable class that was expected to be thrown.Returns:nothing, it's just to be used in `doSomething(optional.orElseGet(() -> shouldHaveThrown(IOException.class)));`.Throws:`[AssertionError](https://docs.oracle.com/javase/8/docs/api/java/lang/AssertionError.html)` - with a message explaining that a [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) of given class was expected to be thrown but had not been.

  * ### setMaxLengthForSingleLineDescription ###

    public static void setMaxLengthForSingleLineDescription(int maxLengthForSingleLineDescription)

    In error messages, sets the threshold when iterable/array formatting will be on one line (if their String description length \<= this parameter) or it will be formatted with one element per line.

     The default value for maxLengthForSingleLineDescription is [80](../configuration/Configuration.html#MAX_LENGTH_FOR_SINGLE_LINE_DESCRIPTION).

     The following array will be formatted on one line as its length \<= 80:

    ```
     String[] greatBooks = array("A Game of Thrones", "The Lord of the Rings", "Assassin's Apprentice");
     // formatted as:
     ["A Game of Thrones", "The Lord of the Rings", "Assassin's Apprentice"]
    ```

     whereas this array is formatted on multiple lines (one element per line)

    ```
     String[] greatBooks = array("A Game of Thrones", "The Lord of the Rings", "Assassin's Apprentice", "Guards! Guards! (Discworld)");
     // formatted as:
     ["A Game of Thrones",
      "The Lord of the Rings",
      "Assassin's Apprentice",
      "Guards! Guards! (Discworld)"]
    ```

    Parameters:`maxLengthForSingleLineDescription` - the maximum length for an iterable/array to be displayed on one line

  * ### setMaxElementsForPrinting ###

    public static void setMaxElementsForPrinting(int maxElementsForPrinting)

    Sets the maximum number of elements to display in error messages for iterables, arrays and map .

     Example with a value of `4`.

     The following array will be formatted entirely as it's length is \<= 4:

    ```
     String[] greatBooks = array("A Game of Thrones", "The Lord of the Rings", "Assassin's Apprentice");
     // formatted as:
     ["A Game of Thrones", "The Lord of the Rings", "Assassin's Apprentice"]
    ```

     whereas for this 6 elements array, only the first and last two elements are displayed (4 in total):

    ```
     String[] greatBooks = array("A Game of Thrones", "The Lord of the Rings", "Assassin's Apprentice", "Guards! Guards!", "The Lies of Locke Lamora", "Aux Ombres d’Abyme");
     // formatted as:
     ["A Game of Thrones", "The Lord of the Rings", ... "The Lies of Locke Lamora", "Aux Ombres d’Abyme"]
    ```

    Parameters:`maxElementsForPrinting` - the maximum elements that would be printed for iterables, arrays and maps.Since:2.6.0 / 3.6.0

  * ### setPrintAssertionsDescription ###

    public static void setPrintAssertionsDescription(boolean printAssertionsDescription)

    Enable/disable printing assertions description to the console (disabled by default).

     The printed assertions description include all the successful assertions description and respectively the first failed one for standard assertions and all failed ones for soft assertions.

     If you want to process the description differently, create a [`Consumer<Description>`](https://docs.oracle.com/javase/8/docs/api/java/util/function/Consumer.html) and register it with [`setDescriptionConsumer(Consumer)`](#setDescriptionConsumer(java.util.function.Consumer)).

    Parameters:`printAssertionsDescription` - whether to print assertions description.Since:3.17.0

  * ### setDescriptionConsumer ###

    public static void setDescriptionConsumer([Consumer](https://docs.oracle.com/javase/8/docs/api/java/util/function/Consumer.html)\<[Description](../description/Description.html)\> descriptionConsumer)

    All assertions description will be consumed by the given [`Consumer<Description>`](https://docs.oracle.com/javase/8/docs/api/java/util/function/Consumer.html) allowing for example to record them in a file.

     The consumed descriptions include all the successful assertions description and respectively the first failed one for standard assertions and all failed ones for soft assertions.

     To unset the descriptionConsumer, call `setDescriptionConsumer(null);`

    Parameters:`descriptionConsumer` - the [`Description`](../description/Description.html) consumerSince:3.17.0

  * ### setMaxStackTraceElementsDisplayed ###

    public static void setMaxStackTraceElementsDisplayed(int maxStackTraceElementsDisplayed)

    Sets how many stacktrace elements are included in [`Throwable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Throwable.html) representation (by default this set to 3).

     Examples:

    ```
      static class Test1 {

       static void boom() {
         Test2.boom2();
       }

       static class Test2 {
         static void boom2() {
           throw new RuntimeException();
         }
       }
     }
    ```

    `Test1.boom()` exception should be represented like this in error messages:

    ```
     java.lang.RuntimeException
            at org.assertj.core.presentation.Test1$Test2.boom2(StandardRepresentation_throwable_format_Test.java:35)
            at org.assertj.core.presentation.Test1.boom(StandardRepresentation_throwable_format_Test.java:40);java.lang.RuntimeException
            at org.assertj.core.presentation.Test1.lambda$1(StandardRepresentation_throwable_format_Test.java:63)org.assertj.core.util.Throwables_Description_Test$test1$test2.exception_layer_2(Throwables_Description_Test.java:24)
            ...(69 remaining lines not displayed - this can be changed with Assertions.setMaxStackTraceElementsDisplayed)org.assertj.core.util.Throwables_Description_Test$test1.exception_layer_1(Throwables_Description_Test.java:30)
    ```

    Parameters:`maxStackTraceElementsDisplayed` - the maximum number of lines for a stacktrace to be displayed on one throw.Since:3.19.0See Also:
    * [`Configuration`](../configuration/Configuration.html)

  * ### extractProperty ###

    public static \<T\> [Properties](../groups/Properties.html)\<T\> extractProperty([String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) propertyName, [Class](https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html)\<T\> propertyType)

    Only delegate to [`Properties.extractProperty(String)`](../groups/Properties.html#extractProperty(java.lang.String)) so that Assertions offers a full feature entry point to all AssertJ features (but you can use [`Properties`](../groups/Properties.html) if you prefer).

     Typical usage is to chain `extractProperty` with `from` method, see examples below :

    ```
     // extract simple property values having a java standard type (here String)
     assertThat(extractProperty("name", String.class).from(fellowshipOfTheRing))
               .contains("Boromir", "Gandalf", "Frodo", "Legolas")
               .doesNotContain("Sauron", "Elrond");

     // extracting property works also with user's types (here Race)
     assertThat(extractProperty("race", String.class).from(fellowshipOfTheRing))
               .contains(HOBBIT, ELF).doesNotContain(ORC);

     // extract nested property on Race
     assertThat(extractProperty("race.name", String.class).from(fellowshipOfTheRing))
               .contains("Hobbit", "Elf")
               .doesNotContain("Orc");
    ```

    Type Parameters:`T` - the type of value to extract.Parameters:`propertyName` - the name of the property to be read from the elements of a `Iterable`. It may be a nested property (e.g. "address.street.number").`propertyType` - the type of property to extractReturns:the created `Properties`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given property name is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given property name is empty.

  * ### extractProperty ###

    public static [Properties](../groups/Properties.html)\<[Object](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html)\> extractProperty([String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) propertyName)

    Only delegate to [`Properties.extractProperty(String)`](../groups/Properties.html#extractProperty(java.lang.String)) so that Assertions offers a full feature entry point to all AssertJ features (but you can use [`Properties`](../groups/Properties.html) if you prefer).

     Typical usage is to chain `extractProperty` with `from` method, see examples below :

    ```
     // extract simple property values, as no type has been defined the extracted property will be considered as Object
     // to define the real property type (here String) use extractProperty("name", String.class) instead.
     assertThat(extractProperty("name").from(fellowshipOfTheRing))
               .contains("Boromir", "Gandalf", "Frodo", "Legolas")
               .doesNotContain("Sauron", "Elrond");

     // extracting property works also with user's types (here Race), even though it will be considered as Object
     // to define the real property type (here String) use extractProperty("name", Race.class) instead.
     assertThat(extractProperty("race").from(fellowshipOfTheRing)).contains(HOBBIT, ELF).doesNotContain(ORC);

     // extract nested property on Race
     assertThat(extractProperty("race.name").from(fellowshipOfTheRing)).contains("Hobbit", "Elf").doesNotContain("Orc");
    ```

    Parameters:`propertyName` - the name of the property to be read from the elements of a `Iterable`. It may be a nested property (e.g. "address.street.number").Returns:the created `Properties`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given property name is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given property name is empty.

  * ### tuple ###

    public static [Tuple](../groups/Tuple.html) tuple([Object](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html)... values)

    Utility method to build nicely a [`Tuple`](../groups/Tuple.html) when working with [`AbstractIterableAssert.extracting(String...)`](AbstractIterableAssert.html#extracting(java.lang.String...)) or [`AbstractObjectArrayAssert.extracting(String...)`](AbstractObjectArrayAssert.html#extracting(java.lang.String...))

    Parameters:`values` - the values stored in the [`Tuple`](../groups/Tuple.html)Returns:the built [`Tuple`](../groups/Tuple.html)

  * ### setAllowExtractingPrivateFields ###

    public static void setAllowExtractingPrivateFields(boolean allowExtractingPrivateFields)

    Globally sets whether `[`IterableAssert#extracting(String)`](AbstractIterableAssert.html#extracting(java.lang.String))` and `[`ObjectArrayAssert#extracting(String)`](AbstractObjectArrayAssert.html#extracting(java.lang.String))` should be allowed to extract private fields, if not and they try it fails with exception.

    Parameters:`allowExtractingPrivateFields` - allow private fields extraction. Default is [true](../configuration/Configuration.html#ALLOW_EXTRACTING_PRIVATE_FIELDS).

  * ### setAllowComparingPrivateFields ###

    public static void setAllowComparingPrivateFields(boolean allowComparingPrivateFields)

    Globally sets whether the use of private fields is allowed for comparison. The following (incomplete) list of methods will be impacted by this change :
    * `[`AbstractIterableAssert.usingElementComparatorOnFields(java.lang.String...)`](AbstractIterableAssert.html#usingElementComparatorOnFields(java.lang.String...))`
    * `[`AbstractObjectAssert.isEqualToComparingFieldByField(Object)`](AbstractObjectAssert.html#isEqualToComparingFieldByField(java.lang.Object))`

     If the value is `false` and these methods try to compare private fields, it will fail with an exception.

    Parameters:`allowComparingPrivateFields` - allow private fields comparison. Default is [true](../configuration/Configuration.html#ALLOW_COMPARING_PRIVATE_FIELDS).

  * ### setExtractBareNamePropertyMethods ###

    public static void setExtractBareNamePropertyMethods(boolean barenamePropertyMethods)

    Globally sets whether the extractor considers bare-named property methods like `String name()`. Defaults to enabled.

    Parameters:`barenamePropertyMethods` - whether bare-named property methods are found

  * ### entry ###

    public static \<K,V\> [MapEntry](../data/MapEntry.html)\<K,V\> entry(K key, V value)

    Only delegate to [`MapEntry.entry(Object, Object)`](../data/MapEntry.html#entry(K,V)) so that Assertions offers a full feature entry point to all AssertJ features (but you can use [`MapEntry`](../data/MapEntry.html) if you prefer).

     Typical usage is to call `entry` in MapAssert `contains` assertion, see examples below :

    ```
     Map<Ring, TolkienCharacter> ringBearers = ... // init omitted

     assertThat(ringBearers).contains(entry(oneRing, frodo), entry(nenya, galadriel));
    ```

    Type Parameters:`K` - the type of keys in the map.`V` - the type of values in the map.Parameters:`key` - the key of the entry to create.`value` - the value of the entry to create.Returns:the created `MapEntry`.

  * ### atIndex ###

    public static [Index](../data/Index.html) atIndex(int index)

    Only delegate to [`Index.atIndex(int)`](../data/Index.html#atIndex(int)) so that Assertions offers a full feature entry point to all AssertJ features (but you can use [`Index`](../data/Index.html) if you prefer).

     Typical usage :

    ```
     List<Ring> elvesRings = newArrayList(vilya, nenya, narya);
     assertThat(elvesRings).contains(vilya, atIndex(0)).contains(nenya, atIndex(1)).contains(narya, atIndex(2));
    ```

    Parameters:`index` - the value of the index.Returns:the created `Index`.Throws:`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### offset ###

    public static [Offset](../data/Offset.html)\<[Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html)\> offset([Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html) value)

    Assertions entry point for double [`Offset`](../data/Offset.html).

     Typical usage :

    ```
     assertThat(0.1).isEqualTo(0.0, offset(0.1));
    ```

    Parameters:`value` - the allowed offsetReturns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### offset ###

    public static [Offset](../data/Offset.html)\<[Float](https://docs.oracle.com/javase/8/docs/api/java/lang/Float.html)\> offset([Float](https://docs.oracle.com/javase/8/docs/api/java/lang/Float.html) value)

    Assertions entry point for float [`Offset`](../data/Offset.html).

     Typical usage :

    ```
     assertThat(0.2f).isCloseTo(0.0f, offset(0.2f));
    ```

    Parameters:`value` - the allowed offsetReturns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### within ###

    public static [Offset](../data/Offset.html)\<[Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html)\> within([Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html) value)

    Alias for [`offset(Double)`](#offset(java.lang.Double)) to use with isCloseTo assertions.

     Typical usage :

    ```
     assertThat(0.1).isCloseTo(0.0, within(0.1));
    ```

    Parameters:`value` - the allowed offsetReturns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### withPrecision ###

    public static [Offset](../data/Offset.html)\<[Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html)\> withPrecision([Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html) value)

    Alias for [`offset(Double)`](#offset(java.lang.Double)) to use with real number assertions.

     Typical usage :

    ```
     assertThat(0.1).isEqualTo(0.0, withPrecision(0.1));
    ```

    Parameters:`value` - the required precisionReturns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### within ###

    public static [Offset](../data/Offset.html)\<[Float](https://docs.oracle.com/javase/8/docs/api/java/lang/Float.html)\> within([Float](https://docs.oracle.com/javase/8/docs/api/java/lang/Float.html) value)

    Alias for [`offset(Float)`](#offset(java.lang.Float)) to use with isCloseTo assertions.

     Typical usage :

    ```
     assertThat(8.2f).isCloseTo(8.0f, within(0.2f));
    ```

    Parameters:`value` - the allowed offsetReturns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### withPrecision ###

    public static [Offset](../data/Offset.html)\<[Float](https://docs.oracle.com/javase/8/docs/api/java/lang/Float.html)\> withPrecision([Float](https://docs.oracle.com/javase/8/docs/api/java/lang/Float.html) value)

    Alias for [`offset(Float)`](#offset(java.lang.Float)) to use with real number assertions.

     Typical usage :

    ```
     assertThat(0.2f).isEqualTo(0.0f, withPrecision(0.2f));
    ```

    Parameters:`value` - the required precisionReturns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### within ###

    public static [Offset](../data/Offset.html)\<[BigDecimal](https://docs.oracle.com/javase/8/docs/api/java/math/BigDecimal.html)\> within([BigDecimal](https://docs.oracle.com/javase/8/docs/api/java/math/BigDecimal.html) value)

    Assertions entry point for BigDecimal [`Offset`](../data/Offset.html) to use with isCloseTo assertions.

     Typical usage :

    ```
     assertThat(BigDecimal.TEN).isCloseTo(new BigDecimal("10.5"), within(BigDecimal.ONE));
    ```

    Parameters:`value` - the allowed offsetReturns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### within ###

    public static [Offset](../data/Offset.html)\<[BigInteger](https://docs.oracle.com/javase/8/docs/api/java/math/BigInteger.html)\> within([BigInteger](https://docs.oracle.com/javase/8/docs/api/java/math/BigInteger.html) value)

    Assertions entry point for BigInteger [`Offset`](../data/Offset.html) to use with isCloseTo assertions.

     Typical usage :

    ```
     assertThat(BigInteger.TEN).isCloseTo(new BigInteger("11"), within(new BigInteger("2")));
    ```

    Parameters:`value` - the allowed offsetReturns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.Since:2.7.0 / 3.7.0, 2.7.0 / 3.7.0

  * ### within ###

    public static [Offset](../data/Offset.html)\<[Byte](https://docs.oracle.com/javase/8/docs/api/java/lang/Byte.html)\> within([Byte](https://docs.oracle.com/javase/8/docs/api/java/lang/Byte.html) value)

    Assertions entry point for Byte [`Offset`](../data/Offset.html) to use with isCloseTo assertions.

     Typical usage :

    ```
     assertThat((byte) 10).isCloseTo((byte) 11, within((byte) 1));
    ```

    Parameters:`value` - the value of the offset.Returns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### within ###

    public static [Offset](../data/Offset.html)\<[Integer](https://docs.oracle.com/javase/8/docs/api/java/lang/Integer.html)\> within([Integer](https://docs.oracle.com/javase/8/docs/api/java/lang/Integer.html) value)

    Assertions entry point for Integer [`Offset`](../data/Offset.html) to use with isCloseTo assertions.

     Typical usage :

    ```
     assertThat(10).isCloseTo(11, within(1));
    ```

    Parameters:`value` - the value of the offset.Returns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### within ###

    public static [Offset](../data/Offset.html)\<[Short](https://docs.oracle.com/javase/8/docs/api/java/lang/Short.html)\> within([Short](https://docs.oracle.com/javase/8/docs/api/java/lang/Short.html) value)

    Assertions entry point for Short [`Offset`](../data/Offset.html) to use with isCloseTo assertions.

     Typical usage :

    ```
     assertThat(10).isCloseTo(11, within(1));
    ```

    Parameters:`value` - the allowed offsetReturns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### within ###

    public static [Offset](../data/Offset.html)\<[Long](https://docs.oracle.com/javase/8/docs/api/java/lang/Long.html)\> within([Long](https://docs.oracle.com/javase/8/docs/api/java/lang/Long.html) value)

    Assertions entry point for Long [`Offset`](../data/Offset.html) to use with [`isCloseTo`](AbstractLongAssert.html#isCloseTo(long,org.assertj.core.data.Offset)) assertions.

     Typical usage :

    ```
     assertThat(5l).isCloseTo(7l, within(2l));
    ```

    Parameters:`value` - the allowed offsetReturns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### within ###

    public static [TemporalUnitOffset](../data/TemporalUnitOffset.html) within(long value, [TemporalUnit](https://docs.oracle.com/javase/8/docs/api/java/time/temporal/TemporalUnit.html) unit)

    Assertions entry point for [`TemporalUnitOffset`](../data/TemporalUnitOffset.html) with less than or equal condition to use with isCloseTo temporal assertions.

     Typical usage :

    ```
     LocalTime _07_10 = LocalTime.of(7, 10);
     LocalTime _07_12 = LocalTime.of(7, 12);
     assertThat(_07_10).isCloseTo(_07_12, within(5, ChronoUnit.MINUTES));
    ```

    Parameters:`value` - the allowed offset`unit` - the [`TemporalUnit`](https://docs.oracle.com/javase/8/docs/api/java/time/temporal/TemporalUnit.html) of the offsetReturns:the created `Offset`.Since:3.7.0See Also:
    * [`byLessThan(long, TemporalUnit)`](#byLessThan(long,java.time.temporal.TemporalUnit))

  * ### withMarginOf ###

    public static [Duration](https://docs.oracle.com/javase/8/docs/api/java/time/Duration.html) withMarginOf([Duration](https://docs.oracle.com/javase/8/docs/api/java/time/Duration.html) allowedDifference)

    Syntactic sugar method to use with [`AbstractDurationAssert.isCloseTo(Duration, Duration)`](AbstractDurationAssert.html#isCloseTo(java.time.Duration,java.time.Duration)) assertion.

     Example:

    ```
     assertThat(Duration.ofMinutes(2)).isCloseTo(Duration.ofMinutes(3), withMarginOf(Duration.ofMinutes(1)));
    ```

    Parameters:`allowedDifference` - the allowed difference [`Duration`](https://docs.oracle.com/javase/8/docs/api/java/time/Duration.html).Returns:the given value.

  * ### withinPercentage ###

    public static [Percentage](../data/Percentage.html) withinPercentage([Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html) value)

    Assertions entry point for Double [`Percentage`](../data/Percentage.html) to use with isCloseTo assertions for percentages.

     Typical usage :

    ```
     assertThat(11.0).isCloseTo(10.0, withinPercentage(10.0));
    ```

    Parameters:`value` - the required precision percentageReturns:the created `Percentage`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### withinPercentage ###

    public static [Percentage](../data/Percentage.html) withinPercentage([Integer](https://docs.oracle.com/javase/8/docs/api/java/lang/Integer.html) value)

    Assertions entry point for Integer [`Percentage`](../data/Percentage.html) to use with isCloseTo assertions for percentages.

     Typical usage :

    ```
     assertThat(11).isCloseTo(10, withinPercentage(10));
    ```

    Parameters:`value` - the required precision percentageReturns:the created `Percentage`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### withinPercentage ###

    public static [Percentage](../data/Percentage.html) withinPercentage([Long](https://docs.oracle.com/javase/8/docs/api/java/lang/Long.html) value)

    Assertions entry point for Long [`Percentage`](../data/Percentage.html) to use with isCloseTo assertions for percentages.

     Typical usage :

    ```
     assertThat(11L).isCloseTo(10L, withinPercentage(10L));
    ```

    Parameters:`value` - the required precision percentageReturns:the created `Percentage`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### byLessThan ###

    public static [Offset](../data/Offset.html)\<[Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html)\> byLessThan([Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html) value)

    Build a [`**strict** Offset`](../data/Offset.html#strictOffset(T)) to use with [`AbstractDoubleAssert.isCloseTo(double, Offset)`](AbstractDoubleAssert.html#isCloseTo(double,org.assertj.core.data.Offset)) and [`AbstractDoubleAssert.isNotCloseTo(double, Offset)`](AbstractDoubleAssert.html#isNotCloseTo(double,org.assertj.core.data.Offset)) assertions.

     A strict offset implies a strict comparison which means that `isCloseTo` will fail when *abs(actual - expected) == offset*.

     Examples:

    ```
     // assertion succeeds
     assertThat(8.1).isCloseTo(8.0, byLessThan(0.2));

     // assertions fail
     assertThat(8.1).isCloseTo(8.0, byLessThan(0.1)); // strict comparison!
     assertThat(8.1).isCloseTo(8.0, byLessThan(0.01));
    ```

    Parameters:`value` - the value of the offset.Returns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### byLessThan ###

    public static [Offset](../data/Offset.html)\<[Float](https://docs.oracle.com/javase/8/docs/api/java/lang/Float.html)\> byLessThan([Float](https://docs.oracle.com/javase/8/docs/api/java/lang/Float.html) value)

    Alias for [`offset(Float)`](#offset(java.lang.Float)) to use with isCloseTo assertions.

     Typical usage :

    ```
     assertThat(8.2f).isCloseTo(8.0f, byLessThan(0.5f));
    ```

    Parameters:`value` - the value of the offset.Returns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### byLessThan ###

    public static [Offset](../data/Offset.html)\<[BigDecimal](https://docs.oracle.com/javase/8/docs/api/java/math/BigDecimal.html)\> byLessThan([BigDecimal](https://docs.oracle.com/javase/8/docs/api/java/math/BigDecimal.html) value)

    Assertions entry point for BigDecimal [`Offset`](../data/Offset.html) to use with isCloseTo assertions.

     Typical usage :

    ```
     assertThat(BigDecimal.TEN).isCloseTo(new BigDecimal("10.5"), byLessThan(BigDecimal.ONE));
    ```

    Parameters:`value` - the value of the offset.Returns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### byLessThan ###

    public static [Offset](../data/Offset.html)\<[BigInteger](https://docs.oracle.com/javase/8/docs/api/java/math/BigInteger.html)\> byLessThan([BigInteger](https://docs.oracle.com/javase/8/docs/api/java/math/BigInteger.html) value)

    Assertions entry point for BigInteger [`Offset`](../data/Offset.html) to use with isCloseTo assertions.

     Typical usage :

    ```
     assertThat(BigInteger.TEN).isCloseTo(new BigInteger("11"), byLessThan(new BigInteger("2")));
    ```

    Parameters:`value` - the value of the offset.Returns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.Since:2.7.0 / 3.7.0

  * ### byLessThan ###

    public static [Offset](../data/Offset.html)\<[Byte](https://docs.oracle.com/javase/8/docs/api/java/lang/Byte.html)\> byLessThan([Byte](https://docs.oracle.com/javase/8/docs/api/java/lang/Byte.html) value)

    Assertions entry point for Byte [`Offset`](../data/Offset.html) to use with isCloseTo assertions.

     Typical usage :

    ```
     assertThat((byte) 10).isCloseTo((byte) 11, byLessThan((byte) 2));
    ```

    Parameters:`value` - the value of the offset.Returns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### byLessThan ###

    public static [Offset](../data/Offset.html)\<[Integer](https://docs.oracle.com/javase/8/docs/api/java/lang/Integer.html)\> byLessThan([Integer](https://docs.oracle.com/javase/8/docs/api/java/lang/Integer.html) value)

    Assertions entry point for Long [`Offset`](../data/Offset.html) to use with strict [`isCloseTo`](AbstractIntegerAssert.html#isCloseTo(int,org.assertj.core.data.Offset)) assertions.

     Typical usage :

    ```
     assertThat(10).isCloseTo(12, byLessThan(1));
    ```

    Parameters:`value` - the value of the offset.Returns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### byLessThan ###

    public static [Offset](../data/Offset.html)\<[Short](https://docs.oracle.com/javase/8/docs/api/java/lang/Short.html)\> byLessThan([Short](https://docs.oracle.com/javase/8/docs/api/java/lang/Short.html) value)

    Assertions entry point for Short [`Offset`](../data/Offset.html) to use with isCloseTo assertions.

     Typical usage :

    ```
     assertThat((short) 10).isCloseTo((short) 11, byLessThan((short) 2));
    ```

    Parameters:`value` - the value of the offset.Returns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### byLessThan ###

    public static [Offset](../data/Offset.html)\<[Long](https://docs.oracle.com/javase/8/docs/api/java/lang/Long.html)\> byLessThan([Long](https://docs.oracle.com/javase/8/docs/api/java/lang/Long.html) value)

    Assertions entry point for Long [`Offset`](../data/Offset.html) to use with strict [`isCloseTo`](AbstractLongAssert.html#isCloseTo(long,org.assertj.core.data.Offset)) assertions.

     Typical usage :

    ```
     assertThat(5l).isCloseTo(7l, byLessThan(3l));
    ```

    Parameters:`value` - the value of the offset.Returns:the created `Offset`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given value is `null`.`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given value is negative.

  * ### byLessThan ###

    public static [TemporalUnitOffset](../data/TemporalUnitOffset.html) byLessThan(long value, [TemporalUnit](https://docs.oracle.com/javase/8/docs/api/java/time/temporal/TemporalUnit.html) unit)

    Assertions entry point for [`TemporalUnitOffset`](../data/TemporalUnitOffset.html) with strict less than condition to use with `isCloseTo` temporal assertions.

     Typical usage :

    ```
     LocalTime _07_10 = LocalTime.of(7, 10);
     LocalTime _07_12 = LocalTime.of(7, 12);
     assertThat(_07_10).isCloseTo(_07_12, byLessThan(5, ChronoUnit.MINUTES));
    ```

    Parameters:`value` - the value of the offset.`unit` - the [`TemporalUnit`](https://docs.oracle.com/javase/8/docs/api/java/time/temporal/TemporalUnit.html) of the offset.Returns:the created `Offset`.Since:3.7.0See Also:
    * [`within(long, TemporalUnit)`](#within(long,java.time.temporal.TemporalUnit))

  * ### from ###

    public static \<F,T\> [Function](https://docs.oracle.com/javase/8/docs/api/java/util/function/Function.html)\<F,T\> from([Function](https://docs.oracle.com/javase/8/docs/api/java/util/function/Function.html)\<F,T\> extractor)

    A syntax sugar to write fluent assertion using [`AbstractObjectAssert.returns(Object, Function)`](AbstractObjectAssert.html#returns(T,java.util.function.Function)) and [`AbstractObjectAssert.doesNotReturn(Object, Function)`](AbstractObjectAssert.html#doesNotReturn(T,java.util.function.Function)).

     Example:

    ```
     Jedi yoda = new Jedi("Yoda", "Green");
     assertThat(yoda).returns("Yoda", from(Jedi::getName))
                     .returns(2.4, from(Jedi::getHeight))
                     .doesNotReturn(null, from(Jedi::getWeight));
    ```

    Type Parameters:`F` - Type of test subject`T` - Type of the property under the assertionParameters:`extractor` - A function to extract test subject's propertyReturns:same instance of `extractor`

  * ### as ###

    public static \<T,ASSERT extends [AbstractAssert](AbstractAssert.html)\<?,?\>\>[InstanceOfAssertFactory](InstanceOfAssertFactory.html)\<T,ASSERT\> as([InstanceOfAssertFactory](InstanceOfAssertFactory.html)\<T,ASSERT\> assertFactory)

    A syntax sugar to write fluent assertion with methods having an [`InstanceOfAssertFactory`](InstanceOfAssertFactory.html) parameter.

     Example:

    ```
     Jedi yoda = new Jedi("Yoda", "Green");
     assertThat(yoda).extracting(Jedi::getName, as(InstanceOfAssertFactories.STRING))
                     .startsWith("Yo");
    ```

    Type Parameters:`T` - the type to use for the cast.`ASSERT` - the type of the resulting `Assert`Parameters:`assertFactory` - the factory which verifies the type and creates the new `Assert`Returns:same instance of `assertFactory`Since:3.14.0See Also:
    * [`AbstractObjectAssert.extracting(String, InstanceOfAssertFactory)`](AbstractObjectAssert.html#extracting(java.lang.String,org.assertj.core.api.InstanceOfAssertFactory))
    * [`AbstractObjectAssert.extracting(Function, InstanceOfAssertFactory)`](AbstractObjectAssert.html#extracting(java.util.function.Function,org.assertj.core.api.InstanceOfAssertFactory))
    * [`AbstractMapAssert.extractingByKey(Object, InstanceOfAssertFactory)`](AbstractMapAssert.html#extractingByKey(K,org.assertj.core.api.InstanceOfAssertFactory))
    * [`AbstractOptionalAssert.get(InstanceOfAssertFactory)`](AbstractOptionalAssert.html#get(org.assertj.core.api.InstanceOfAssertFactory))
    * [`AbstractIterableAssert.first(InstanceOfAssertFactory)`](AbstractIterableAssert.html#first(org.assertj.core.api.InstanceOfAssertFactory))
    * [`AbstractIterableAssert.last(InstanceOfAssertFactory)`](AbstractIterableAssert.html#last(org.assertj.core.api.InstanceOfAssertFactory))
    * [`AbstractIterableAssert.element(int, InstanceOfAssertFactory)`](AbstractIterableAssert.html#element(int,org.assertj.core.api.InstanceOfAssertFactory))

  * ### allOf ###

    [@SafeVarargs](https://docs.oracle.com/javase/8/docs/api/java/lang/SafeVarargs.html)public static \<T\> [Condition](Condition.html)\<T\> allOf([Condition](Condition.html)\<? super T\>... conditions)

    Creates a new `[`AllOf`](../condition/AllOf.html)`

    Type Parameters:`T` - the type of object the given condition accept.Parameters:`conditions` - the conditions to evaluate.Returns:the created `AllOf`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given array is `null`.`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if any of the elements in the given array is `null`.

  * ### allOf ###

    public static \<T\> [Condition](Condition.html)\<T\> allOf([Iterable](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html)\<? extends [Condition](Condition.html)\<? super T\>\> conditions)

    Creates a new `[`AllOf`](../condition/AllOf.html)`

    Type Parameters:`T` - the type of object the given condition accept.Parameters:`conditions` - the conditions to evaluate.Returns:the created `AllOf`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given iterable is `null`.`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if any of the elements in the given iterable is `null`.

  * ### allOf ###

    [@SafeVarargs](https://docs.oracle.com/javase/8/docs/api/java/lang/SafeVarargs.html)public static \<T\> [ThrowingConsumer](ThrowingConsumer.html)\<T\> allOf([ThrowingConsumer](ThrowingConsumer.html)\<? super T\>... consumers)

    Create a new `[`ThrowingConsumer`](ThrowingConsumer.html)` that delegates the evaluation of the given consumers to [`AbstractAssert.satisfies(ThrowingConsumer[])`](AbstractAssert.html#satisfies(org.assertj.core.api.ThrowingConsumer...)).

    Type Parameters:`T` - the type of object the given consumers acceptParameters:`consumers` - the consumers to evaluateReturns:the `ThrowingConsumer` instanceSince:3.25.0

  * ### anyOf ###

    [@SafeVarargs](https://docs.oracle.com/javase/8/docs/api/java/lang/SafeVarargs.html)public static \<T\> [Condition](Condition.html)\<T\> anyOf([Condition](Condition.html)\<? super T\>... conditions)

    Only delegate to [`AnyOf.anyOf(Condition...)`](../condition/AnyOf.html#anyOf(org.assertj.core.api.Condition...)) so that Assertions offers a full feature entry point to all AssertJ features (but you can use [`AnyOf`](../condition/AnyOf.html) if you prefer).

     Typical usage (`jedi` and `sith` are [`Condition`](Condition.html)) :

    ```
     assertThat("Vader").is(anyOf(jedi, sith));
    ```

    Type Parameters:`T` - the type of object the given condition accept.Parameters:`conditions` - the conditions to evaluate.Returns:the created `AnyOf`.

  * ### anyOf ###

    public static \<T\> [Condition](Condition.html)\<T\> anyOf([Iterable](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html)\<? extends [Condition](Condition.html)\<? super T\>\> conditions)

    Creates a new `[`AnyOf`](../condition/AnyOf.html)`

    Type Parameters:`T` - the type of object the given condition accept.Parameters:`conditions` - the conditions to evaluate.Returns:the created `AnyOf`.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given iterable is `null`.`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if any of the elements in the given iterable is `null`.

  * ### anyOf ###

    [@SafeVarargs](https://docs.oracle.com/javase/8/docs/api/java/lang/SafeVarargs.html)public static \<T\> [ThrowingConsumer](ThrowingConsumer.html)\<T\> anyOf([ThrowingConsumer](ThrowingConsumer.html)\<? super T\>... consumers)

    Create a new `[`ThrowingConsumer`](ThrowingConsumer.html)` that delegates the evaluation of the given consumers to [`AbstractAssert.satisfiesAnyOf(ThrowingConsumer[])`](AbstractAssert.html#satisfiesAnyOf(org.assertj.core.api.ThrowingConsumer...)).

    Type Parameters:`T` - the type of object the given consumers acceptParameters:`consumers` - the consumers to evaluateReturns:the `ThrowingConsumer` instanceSince:3.25.0

  * ### doesNotHave ###

    public static \<T\> [DoesNotHave](../condition/DoesNotHave.html)\<T\> doesNotHave([Condition](Condition.html)\<? super T\> condition)

    Creates a new `[`DoesNotHave`](../condition/DoesNotHave.html)`.

    Type Parameters:`T` - the type of object the given condition accept.Parameters:`condition` - the condition to inverse.Returns:The DoesNotHave condition created.

  * ### not ###

    public static \<T\> [Not](../condition/Not.html)\<T\> not([Condition](Condition.html)\<? super T\> condition)

    Creates a new `[`Not`](../condition/Not.html)`.

    Type Parameters:`T` - the type of object the given condition accept.Parameters:`condition` - the condition to inverse.Returns:The Not condition created.

  * ### filter ###

    public static \<E\> [Filters](filter/Filters.html)\<E\> filter(E[] array)

    Only delegate to [`Filters.filter(Object[])`](filter/Filters.html#filter(E%5B%5D)) so that Assertions offers a full feature entry point to all AssertJ features (but you can use [`Filters`](filter/Filters.html) if you prefer).

     Note that the given array is not modified, the filters are performed on an [`Iterable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html) copy of the array.

     Typical usage with [`Condition`](Condition.html) :

    ```
     assertThat(filter(players).being(potentialMVP).get()).containsOnly(james, rose);
    ```

     and with filter language based on java bean property :

    ```
     assertThat(filter(players).with("pointsPerGame").greaterThan(20).and("assistsPerGame").greaterThan(7).get())
               .containsOnly(james, rose);
    ```

    Type Parameters:`E` - the array elements type.Parameters:`array` - the array to filter.Returns:the created `[`Filters`](filter/Filters.html)`.

  * ### filter ###

    public static \<E\> [Filters](filter/Filters.html)\<E\> filter([Iterable](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html)\<E\> iterableToFilter)

    Only delegate to [`Filters.filter(Object[])`](filter/Filters.html#filter(E%5B%5D)) so that Assertions offers a full feature entry point to all AssertJ features (but you can use [`Filters`](filter/Filters.html) if you prefer).

     Note that the given [`Iterable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html) is not modified, the filters are performed on a copy.

     Typical usage with [`Condition`](Condition.html) :

    ```
     assertThat(filter(players).being(potentialMVP).get()).containsOnly(james, rose);
    ```

     and with filter language based on java bean property :

    ```
     assertThat(filter(players).with("pointsPerGame").greaterThan(20)
                               .and("assistsPerGame").greaterThan(7).get())
               .containsOnly(james, rose);
    ```

    Type Parameters:`E` - the [`Iterable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html) elements type.Parameters:`iterableToFilter` - the `Iterable` to filter.Returns:the created `[`Filters`](filter/Filters.html)`.

  * ### in ###

    public static [InFilter](filter/InFilter.html) in([Object](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html)... values)

    Create a [`FilterOperator`](filter/FilterOperator.html) to use in [`filteredOn(String, FilterOperation)`](AbstractIterableAssert.html#filteredOn(java.lang.String,org.assertj.core.api.filter.FilterOperator)) to express a filter keeping all Iterable elements whose property/field value matches one of the given values.

     As often, an example helps:

    ```
     Employee yoda   = new Employee(1L, new Name("Yoda"), 800);
     Employee obiwan = new Employee(2L, new Name("Obiwan"), 800);
     Employee luke   = new Employee(3L, new Name("Luke", "Skywalker"), 26);
     Employee noname = new Employee(4L, null, 50);

     List<Employee> employees = newArrayList(yoda, luke, obiwan, noname);

     assertThat(employees).filteredOn("age", in(800, 26))
                          .containsOnly(yoda, obiwan, luke);
    ```

    Parameters:`values` - values to match (one match is sufficient)Returns:the created "in" filter

  * ### notIn ###

    public static [NotInFilter](filter/NotInFilter.html) notIn([Object](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html)... valuesNotToMatch)

    Create a [`FilterOperator`](filter/FilterOperator.html) to use in [`filteredOn(String, FilterOperation)`](AbstractIterableAssert.html#filteredOn(java.lang.String,org.assertj.core.api.filter.FilterOperator)) to express a filter keeping all Iterable elements whose property/field value matches does not match any of the given values.

     As often, an example helps:

    ```
     Employee yoda   = new Employee(1L, new Name("Yoda"), 800);
     Employee obiwan = new Employee(2L, new Name("Obiwan"), 800);
     Employee luke   = new Employee(3L, new Name("Luke", "Skywalker"), 26);
     Employee noname = new Employee(4L, null, 50);

     List<Employee> employees = newArrayList(yoda, luke, obiwan, noname);

     assertThat(employees).filteredOn("age", notIn(800, 50))
                          .containsOnly(luke);
    ```

    Parameters:`valuesNotToMatch` - values not to match (none of the values must match)Returns:the created "not in" filter

  * ### not ###

    public static [NotFilter](filter/NotFilter.html) not([Object](https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html) valueNotToMatch)

    Create a [`FilterOperator`](filter/FilterOperator.html) to use in [`filteredOn(String, FilterOperation)`](AbstractIterableAssert.html#filteredOn(java.lang.String,org.assertj.core.api.filter.FilterOperator)) to express a filter keeping all Iterable elements whose property/field value matches does not match the given value.

     As often, an example helps:

    ```
     Employee yoda   = new Employee(1L, new Name("Yoda"), 800);
     Employee obiwan = new Employee(2L, new Name("Obiwan"), 800);
     Employee luke   = new Employee(3L, new Name("Luke", "Skywalker"), 26);
     Employee noname = new Employee(4L, null, 50);

     List<Employee> employees = newArrayList(yoda, luke, obiwan, noname);

     assertThat(employees).filteredOn("age", not(800))
                          .containsOnly(luke, noname);
    ```

    Parameters:`valueNotToMatch` - the value not to matchReturns:the created "not" filter

  * ### contentOf ###

    public static [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) contentOf([File](https://docs.oracle.com/javase/8/docs/api/java/io/File.html) file, [Charset](https://docs.oracle.com/javase/8/docs/api/java/nio/charset/Charset.html) charset)

    Loads the text content of a file, so that it can be passed to [`assertThat(String)`](#assertThat(java.lang.String)).

     Note that this will load the entire file in memory; for larger files, there might be a more efficient alternative with [`assertThat(File)`](#assertThat(java.io.File)).

    Parameters:`file` - the file.`charset` - the character set to use.Returns:the content of the file.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given charset is `null`.`[UncheckedIOException](https://docs.oracle.com/javase/8/docs/api/java/io/UncheckedIOException.html)` - if an I/O exception occurs.

  * ### contentOf ###

    public static [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) contentOf([File](https://docs.oracle.com/javase/8/docs/api/java/io/File.html) file, [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) charsetName)

    Loads the text content of a file, so that it can be passed to [`assertThat(String)`](#assertThat(java.lang.String)).

     Note that this will load the entire file in memory; for larger files, there might be a more efficient alternative with [`assertThat(File)`](#assertThat(java.io.File)).

    Parameters:`file` - the file.`charsetName` - the name of the character set to use.Returns:the content of the file.Throws:`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given character set is not supported on this platform.`[UncheckedIOException](https://docs.oracle.com/javase/8/docs/api/java/io/UncheckedIOException.html)` - if an I/O exception occurs.

  * ### contentOf ###

    public static [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) contentOf([File](https://docs.oracle.com/javase/8/docs/api/java/io/File.html) file)

    Loads the text content of a file with the default character set, so that it can be passed to [`assertThat(String)`](#assertThat(java.lang.String)).

     Note that this will load the entire file in memory; for larger files, there might be a more efficient alternative with [`assertThat(File)`](#assertThat(java.io.File)).

    Parameters:`file` - the file.Returns:the content of the file.Throws:`[UncheckedIOException](https://docs.oracle.com/javase/8/docs/api/java/io/UncheckedIOException.html)` - if an I/O exception occurs.

  * ### linesOf ###

    public static [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)\<[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)\> linesOf([File](https://docs.oracle.com/javase/8/docs/api/java/io/File.html) file)

    Loads the text content of a file into a list of strings with the default charset, each string corresponding to a line. The line endings are either \\n, \\r or \\r\\n.

    Parameters:`file` - the file.Returns:the content of the file.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given charset is `null`.`[UncheckedIOException](https://docs.oracle.com/javase/8/docs/api/java/io/UncheckedIOException.html)` - if an I/O exception occurs.

  * ### linesOf ###

    public static [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)\<[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)\> linesOf([File](https://docs.oracle.com/javase/8/docs/api/java/io/File.html) file, [Charset](https://docs.oracle.com/javase/8/docs/api/java/nio/charset/Charset.html) charset)

    Loads the text content of a file into a list of strings, each string corresponding to a line. The line endings are either \\n, \\r or \\r\\n.

    Parameters:`file` - the file.`charset` - the character set to use.Returns:the content of the file.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given charset is `null`.`[UncheckedIOException](https://docs.oracle.com/javase/8/docs/api/java/io/UncheckedIOException.html)` - if an I/O exception occurs.

  * ### linesOf ###

    public static [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)\<[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)\> linesOf([File](https://docs.oracle.com/javase/8/docs/api/java/io/File.html) file, [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) charsetName)

    Loads the text content of a file into a list of strings, each string corresponding to a line. The line endings are either \\n, \\r or \\r\\n.

    Parameters:`file` - the file.`charsetName` - the name of the character set to use.Returns:the content of the file.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given charset is `null`.`[UncheckedIOException](https://docs.oracle.com/javase/8/docs/api/java/io/UncheckedIOException.html)` - if an I/O exception occurs.

  * ### linesOf ###

    public static [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)\<[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)\> linesOf([Path](https://docs.oracle.com/javase/8/docs/api/java/nio/file/Path.html) path)

    Loads the text content of a file at a given path into a list of strings with the default charset, each string corresponding to a line. The line endings are either \\n, \\r or \\r\\n.

    Parameters:`path` - the path.Returns:the content of the file at the given path.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given charset is `null`.`[UncheckedIOException](https://docs.oracle.com/javase/8/docs/api/java/io/UncheckedIOException.html)` - if an I/O exception occurs.Since:3.23.0

  * ### linesOf ###

    public static [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)\<[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)\> linesOf([Path](https://docs.oracle.com/javase/8/docs/api/java/nio/file/Path.html) path, [Charset](https://docs.oracle.com/javase/8/docs/api/java/nio/charset/Charset.html) charset)

    Loads the text content of a file at a given path into a list of strings, each string corresponding to a line. The line endings are either \\n, \\r or \\r\\n.

    Parameters:`path` - the path.`charset` - the character set to use.Returns:the content of the file at the given path.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given charset is `null`.`[UncheckedIOException](https://docs.oracle.com/javase/8/docs/api/java/io/UncheckedIOException.html)` - if an I/O exception occurs.Since:3.23.0

  * ### linesOf ###

    public static [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)\<[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)\> linesOf([Path](https://docs.oracle.com/javase/8/docs/api/java/nio/file/Path.html) path, [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) charsetName)

    Loads the text content of a file at a given path into a list of strings, each string corresponding to a line. The line endings are either \\n, \\r or \\r\\n.

    Parameters:`path` - the path.`charsetName` - the name of the character set to use.Returns:the content of the file at the given path.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given charset is `null`.`[UncheckedIOException](https://docs.oracle.com/javase/8/docs/api/java/io/UncheckedIOException.html)` - if an I/O exception occurs.Since:3.23.0

  * ### contentOf ###

    public static [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) contentOf([URL](https://docs.oracle.com/javase/8/docs/api/java/net/URL.html) url, [Charset](https://docs.oracle.com/javase/8/docs/api/java/nio/charset/Charset.html) charset)

    Loads the text content of a URL, so that it can be passed to [`assertThat(String)`](#assertThat(java.lang.String)).

     Note that this will load the entire contents in memory.

    Parameters:`url` - the URL.`charset` - the character set to use.Returns:the content of the URL.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given charset is `null`.`[UncheckedIOException](https://docs.oracle.com/javase/8/docs/api/java/io/UncheckedIOException.html)` - if an I/O exception occurs.

  * ### contentOf ###

    public static [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) contentOf([URL](https://docs.oracle.com/javase/8/docs/api/java/net/URL.html) url, [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) charsetName)

    Loads the text content of a URL, so that it can be passed to [`assertThat(String)`](#assertThat(java.lang.String)).

     Note that this will load the entire contents in memory.

    Parameters:`url` - the URL.`charsetName` - the name of the character set to use.Returns:the content of the URL.Throws:`[IllegalArgumentException](https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalArgumentException.html)` - if the given character set is not supported on this platform.`[UncheckedIOException](https://docs.oracle.com/javase/8/docs/api/java/io/UncheckedIOException.html)` - if an I/O exception occurs.

  * ### contentOf ###

    public static [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) contentOf([URL](https://docs.oracle.com/javase/8/docs/api/java/net/URL.html) url)

    Loads the text content of a URL with the default character set, so that it can be passed to [`assertThat(String)`](#assertThat(java.lang.String)).

     Note that this will load the entire file in memory; for larger files.

    Parameters:`url` - the URL.Returns:the content of the file.Throws:`[UncheckedIOException](https://docs.oracle.com/javase/8/docs/api/java/io/UncheckedIOException.html)` - if an I/O exception occurs.

  * ### linesOf ###

    public static [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)\<[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)\> linesOf([URL](https://docs.oracle.com/javase/8/docs/api/java/net/URL.html) url)

    Loads the text content of a URL into a list of strings with the default charset, each string corresponding to a line. The line endings are either \\n, \\r or \\r\\n.

    Parameters:`url` - the URL.Returns:the content of the file.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given charset is `null`.`[UncheckedIOException](https://docs.oracle.com/javase/8/docs/api/java/io/UncheckedIOException.html)` - if an I/O exception occurs.

  * ### linesOf ###

    public static [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)\<[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)\> linesOf([URL](https://docs.oracle.com/javase/8/docs/api/java/net/URL.html) url, [Charset](https://docs.oracle.com/javase/8/docs/api/java/nio/charset/Charset.html) charset)

    Loads the text content of a URL into a list of strings, each string corresponding to a line. The line endings are either \\n, \\r or \\r\\n.

    Parameters:`url` - the URL.`charset` - the character set to use.Returns:the content of the file.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given charset is `null`.`[UncheckedIOException](https://docs.oracle.com/javase/8/docs/api/java/io/UncheckedIOException.html)` - if an I/O exception occurs.

  * ### linesOf ###

    public static [List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)\<[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)\> linesOf([URL](https://docs.oracle.com/javase/8/docs/api/java/net/URL.html) url, [String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) charsetName)

    Loads the text content of a URL into a list of strings, each string corresponding to a line. The line endings are either \\n, \\r or \\r\\n.

    Parameters:`url` - the URL.`charsetName` - the name of the character set to use.Returns:the content of the file.Throws:`[NullPointerException](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html)` - if the given charset is `null`.`[UncheckedIOException](https://docs.oracle.com/javase/8/docs/api/java/io/UncheckedIOException.html)` - if an I/O exception occurs.

  * ### setLenientDateParsing ###

    public static void setLenientDateParsing(boolean value)

    Instead of using default strict date/time parsing, it is possible to use lenient parsing mode for default date formats parser to interpret inputs that do not precisely match supported date formats (lenient parsing).

     With strict parsing, inputs must match exactly date/time format.

     Example:

    ```
     final Date date = Dates.parse("2001-02-03");
     final Date dateTime = parseDatetime("2001-02-03T04:05:06");
     final Date dateTimeWithMs = parseDatetimeWithMs("2001-02-03T04:05:06.700");

     Assertions.setLenientDateParsing(true);

     // assertions will pass
     assertThat(date).isEqualTo("2001-01-34");
     assertThat(date).isEqualTo("2001-02-02T24:00:00");
     assertThat(date).isEqualTo("2001-02-04T-24:00:00.000");
     assertThat(dateTime).isEqualTo("2001-02-03T04:05:05.1000");
     assertThat(dateTime).isEqualTo("2001-02-03T04:04:66");
     assertThat(dateTimeWithMs).isEqualTo("2001-02-03T04:05:07.-300");

     // assertions will fail
     assertThat(date).hasSameTimeAs("2001-02-04"); // different date
     assertThat(dateTime).hasSameTimeAs("2001-02-03 04:05:06"); // leniency does not help here
    ```

     To revert to default strict date parsing, call `setLenientDateParsing(false)`.

    Parameters:`value` - whether lenient parsing mode should be enabled or not

  * ### registerCustomDateFormat ###

    public static void registerCustomDateFormat([DateFormat](https://docs.oracle.com/javase/8/docs/api/java/text/DateFormat.html) userCustomDateFormat)

    Add the given date format to the ones used to parse date String in String based Date assertions like [`AbstractDateAssert.isEqualTo(String)`](AbstractDateAssert.html#isEqualTo(java.lang.String)).

     User date formats are used before default ones in the order they have been registered (first registered, first used).

     AssertJ is gonna use any date formats registered with one of these methods :

    * [`AbstractDateAssert.withDateFormat(String)`](AbstractDateAssert.html#withDateFormat(java.lang.String))
    * [`AbstractDateAssert.withDateFormat(java.text.DateFormat)`](AbstractDateAssert.html#withDateFormat(java.text.DateFormat))
    * [`registerCustomDateFormat(java.text.DateFormat)`](#registerCustomDateFormat(java.text.DateFormat))
    * [`registerCustomDateFormat(String)`](#registerCustomDateFormat(java.lang.String))

     Beware that AssertJ will use the newly registered format for **all remaining Date assertions in the test suite**

     To revert to default formats only, call [`useDefaultDateFormatsOnly()`](#useDefaultDateFormatsOnly()) or [`AbstractDateAssert.withDefaultDateFormatsOnly()`](AbstractDateAssert.html#withDefaultDateFormatsOnly()).

     Code examples:

    ```
     Date date = ... // set to 2003 April the 26th
     assertThat(date).isEqualTo("2003-04-26");

     try {
       // date with a custom format : failure since the default formats don't match.
       assertThat(date).isEqualTo("2003/04/26");
     } catch (AssertionError e) {
       assertThat(e).hasMessage("Failed to parse 2003/04/26 with any of these date formats: " +
                                "[yyyy-MM-dd'T'HH:mm:ss.SSSX, yyyy-MM-dd'T'HH:mm:ss.SSS, " +
                                "yyyy-MM-dd'T'HH:mm:ssX, " +
                                "yyyy-MM-dd'T'HH:mm:ss, yyyy-MM-dd]");
     }

     // registering a custom date format to make the assertion pass
     registerCustomDateFormat(new SimpleDateFormat("yyyy/MM/dd")); // registerCustomDateFormat("yyyy/MM/dd") would work to.
     assertThat(date).isEqualTo("2003/04/26");

     // the default formats are still available and should work
     assertThat(date).isEqualTo("2003-04-26");
    ```

    Parameters:`userCustomDateFormat` - the new Date format used for String based Date assertions.

  * ### registerCustomDateFormat ###

    public static void registerCustomDateFormat([String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) userCustomDateFormatPattern)

    Add the given date format to the ones used to parse date String in String based Date assertions like [`AbstractDateAssert.isEqualTo(String)`](AbstractDateAssert.html#isEqualTo(java.lang.String)).

     User date formats are used before default ones in the order they have been registered (first registered, first used).

     AssertJ is gonna use any date formats registered with one of these methods :

    * [`AbstractDateAssert.withDateFormat(String)`](AbstractDateAssert.html#withDateFormat(java.lang.String))
    * [`AbstractDateAssert.withDateFormat(java.text.DateFormat)`](AbstractDateAssert.html#withDateFormat(java.text.DateFormat))
    * [`registerCustomDateFormat(java.text.DateFormat)`](#registerCustomDateFormat(java.text.DateFormat))
    * [`registerCustomDateFormat(String)`](#registerCustomDateFormat(java.lang.String))

     Beware that AssertJ will use the newly registered format for **all remaining Date assertions in the test suite**.

     To revert to default formats only, call [`useDefaultDateFormatsOnly()`](#useDefaultDateFormatsOnly()) or [`AbstractDateAssert.withDefaultDateFormatsOnly()`](AbstractDateAssert.html#withDefaultDateFormatsOnly()).

     Code examples:

    ```
     Date date = ... // set to 2003 April the 26th
     assertThat(date).isEqualTo("2003-04-26");

     try {
       // date with a custom format : failure since the default formats don't match.
       assertThat(date).isEqualTo("2003/04/26");
     } catch (AssertionError e) {
       assertThat(e).hasMessage("Failed to parse 2003/04/26 with any of these date formats: " +
                                "[yyyy-MM-dd'T'HH:mm:ss.SSSX, yyyy-MM-dd'T'HH:mm:ss.SSS, " +
                                "yyyy-MM-dd'T'HH:mm:ssX, " +
                                "yyyy-MM-dd'T'HH:mm:ss, yyyy-MM-dd]");
     }

     // registering a custom date format to make the assertion pass
     registerCustomDateFormat("yyyy/MM/dd");
     assertThat(date).isEqualTo("2003/04/26");

     // the default formats are still available and should work
     assertThat(date).isEqualTo("2003-04-26");
    ```

    Parameters:`userCustomDateFormatPattern` - the new Date format pattern used for String based Date assertions.

  * ### useDefaultDateFormatsOnly ###

    public static void useDefaultDateFormatsOnly()

    Remove all registered custom date formats =\> use only the defaults date formats to parse string as date.

     Beware that the default formats are expressed in the current local timezone.

     Defaults date format are:

    * `yyyy-MM-dd HH:mm:ss.SSSX`
    * `yyyy-MM-dd'T'HH:mm:ss.SSS`
    * `yyyy-MM-dd HH:mm:ss.SSS` (for [`Timestamp`](https://docs.oracle.com/javase/8/docs/api/java/sql/Timestamp.html) String representation support)
    * `yyyy-MM-dd'T'HH:mm:ssX`
    * `yyyy-MM-dd'T'HH:mm:ss`
    * `yyyy-MM-dd`

     Example of valid string date representations:

    * `2003-04-26T03:01:02.999`
    * `2003-04-26 03:01:02.999`
    * `2003-04-26T13:01:02`
    * `2003-04-26`

  * ### assertThat ###

    public static \<T\> T assertThat([AssertProvider](AssertProvider.html)\<T\> component)

    Delegates the creation of the [`Assert`](Assert.html) to the [`AssertProvider.assertThat()`](AssertProvider.html#assertThat()) of the given component.

     Read the comments on [`AssertProvider`](AssertProvider.html) for an example of its usage.

    Type Parameters:`T` - the AssertProvider wrapped type.Parameters:`component` - the component that creates its own assertReturns:the associated [`Assert`](Assert.html) of the given component

  * ### assertThat ###

    public static [AbstractCharSequenceAssert](AbstractCharSequenceAssert.html)\<?,? extends [CharSequence](https://docs.oracle.com/javase/8/docs/api/java/lang/CharSequence.html)\> assertThat([CharSequence](https://docs.oracle.com/javase/8/docs/api/java/lang/CharSequence.html) actual)

    Creates a new instance of `[`CharSequenceAssert`](CharSequenceAssert.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThatCharSequence ###

    public static [AbstractCharSequenceAssert](AbstractCharSequenceAssert.html)\<?,? extends [CharSequence](https://docs.oracle.com/javase/8/docs/api/java/lang/CharSequence.html)\> assertThatCharSequence([CharSequence](https://docs.oracle.com/javase/8/docs/api/java/lang/CharSequence.html) actual)

    Creates a new instance of `[`CharSequenceAssert`](CharSequenceAssert.html)`.

     Use this over [`assertThat(CharSequence)`](#assertThat(java.lang.CharSequence)) in case of ambiguous method resolution when the object under test implements several interfaces Assertj provides `assertThat` for.

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.25.0

  * ### assertThat ###

    public static [AbstractCharSequenceAssert](AbstractCharSequenceAssert.html)\<?,? extends [CharSequence](https://docs.oracle.com/javase/8/docs/api/java/lang/CharSequence.html)\> assertThat([StringBuilder](https://docs.oracle.com/javase/8/docs/api/java/lang/StringBuilder.html) actual)

    Creates a new instance of `[`CharSequenceAssert`](CharSequenceAssert.html)` from a [`StringBuilder`](https://docs.oracle.com/javase/8/docs/api/java/lang/StringBuilder.html).

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.11.0

  * ### assertThat ###

    public static [AbstractCharSequenceAssert](AbstractCharSequenceAssert.html)\<?,? extends [CharSequence](https://docs.oracle.com/javase/8/docs/api/java/lang/CharSequence.html)\> assertThat([StringBuffer](https://docs.oracle.com/javase/8/docs/api/java/lang/StringBuffer.html) actual)

    Creates a new instance of `[`CharSequenceAssert`](CharSequenceAssert.html)` from a [`StringBuffer`](https://docs.oracle.com/javase/8/docs/api/java/lang/StringBuffer.html).

    Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.11.0

  * ### assertThat ###

    public static [AbstractStringAssert](AbstractStringAssert.html)\<?\> assertThat([String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html) actual)

    Creates a new instance of `[`StringAssert`](StringAssert.html)from a [`String`](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)`.

    Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static \<ELEMENT\> [IterableAssert](IterableAssert.html)\<ELEMENT\> assertThat([Iterable](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html)\<? extends ELEMENT\> actual)

    Creates a new instance of `[`IterableAssert`](IterableAssert.html)`.

    Type Parameters:`ELEMENT` - the type of elements.Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThatIterable ###

    public static \<ELEMENT\> [IterableAssert](IterableAssert.html)\<ELEMENT\> assertThatIterable([Iterable](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html)\<? extends ELEMENT\> actual)

    Creates a new instance of `[`IterableAssert`](IterableAssert.html)`.

     Use this over [`assertThat(Iterable)`](#assertThat(java.lang.Iterable)) in case of ambiguous method resolution when the object under test implements several interfaces Assertj provides `assertThat` for.

    Type Parameters:`ELEMENT` - the type of elements.Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.23.0

  * ### assertThat ###

    public static \<ELEMENT\> [IteratorAssert](IteratorAssert.html)\<ELEMENT\> assertThat([Iterator](https://docs.oracle.com/javase/8/docs/api/java/util/Iterator.html)\<? extends ELEMENT\> actual)

    Creates a new instance of `[`IteratorAssert`](IteratorAssert.html)`.

    **Breaking change in version 3.12.0:** this method does not return anymore an [`IterableAssert`](IterableAssert.html) but an [`IteratorAssert`](IteratorAssert.html).  
     In order to access assertions from [`IterableAssert`](IterableAssert.html), use [`AbstractIteratorAssert.toIterable()`](AbstractIteratorAssert.html#toIterable()).

    [`IteratorAssert`](IteratorAssert.html) instances have limited assertions because it does not consume iterator's elements.

     Examples:

    ```
     Iterator<String> bestBasketBallPlayers = getBestBasketBallPlayers();

     assertThat(bestBasketBallPlayers).hasNext() // Iterator assertion
                                      .toIterable() // switch to Iterable assertions
                                      .contains("Jordan", "Magic", "Lebron"); // Iterable assertion
    ```

    Type Parameters:`ELEMENT` - the type of elements.Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThatIterator ###

    public static \<ELEMENT\> [IteratorAssert](IteratorAssert.html)\<ELEMENT\> assertThatIterator([Iterator](https://docs.oracle.com/javase/8/docs/api/java/util/Iterator.html)\<? extends ELEMENT\> actual)

    Creates a new instance of `[`IteratorAssert`](IteratorAssert.html)`.

     Use this over [`assertThat(Iterator)`](#assertThat(java.util.Iterator)) in case of ambiguous method resolution when the object under test implements several interfaces Assertj provides `assertThat` for.

    Type Parameters:`ELEMENT` - the type of elements.Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.23.0

  * ### assertThat ###

    public static \<E\>[AbstractCollectionAssert](AbstractCollectionAssert.html)\<?,[Collection](https://docs.oracle.com/javase/8/docs/api/java/util/Collection.html)\<? extends E\>,E,[ObjectAssert](ObjectAssert.html)\<E\>\> assertThat([Collection](https://docs.oracle.com/javase/8/docs/api/java/util/Collection.html)\<? extends E\> actual)

    Creates a new instance of `[`CollectionAssert`](CollectionAssert.html)`.

    Type Parameters:`E` - the type of elements.Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.21.0

  * ### assertThatCollection ###

    public static \<E\>[AbstractCollectionAssert](AbstractCollectionAssert.html)\<?,[Collection](https://docs.oracle.com/javase/8/docs/api/java/util/Collection.html)\<? extends E\>,E,[ObjectAssert](ObjectAssert.html)\<E\>\> assertThatCollection([Collection](https://docs.oracle.com/javase/8/docs/api/java/util/Collection.html)\<? extends E\> actual)

    Creates a new instance of `[`CollectionAssert`](CollectionAssert.html)`.

     Use this over [`assertThat(Collection)`](#assertThat(java.util.Collection)) in case of ambiguous method resolution when the object under test implements several interfaces Assertj provides `assertThat` for.

    Type Parameters:`E` - the type of elements.Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.23.0

  * ### assertThat ###

    public static \<ELEMENT\> [ListAssert](ListAssert.html)\<ELEMENT\> assertThat([List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)\<? extends ELEMENT\> actual)

    Creates a new instance of `[`ListAssert`](ListAssert.html)`.

    Type Parameters:`ELEMENT` - the type of elements.Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThatList ###

    public static \<ELEMENT\> [ListAssert](ListAssert.html)\<ELEMENT\> assertThatList([List](https://docs.oracle.com/javase/8/docs/api/java/util/List.html)\<? extends ELEMENT\> actual)

    Creates a new instance of `[`ListAssert`](ListAssert.html)`.

     Use this over [`assertThat(List)`](#assertThat(java.util.List)) in case of ambiguous method resolution when the object under test implements several interfaces Assertj provides `assertThat` for.

    Type Parameters:`ELEMENT` - the type of elements.Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.23.0

  * ### assertThat ###

    public static \<ELEMENT\> [ListAssert](ListAssert.html)\<ELEMENT\> assertThat([Stream](https://docs.oracle.com/javase/8/docs/api/java/util/stream/Stream.html)\<? extends ELEMENT\> actual)

    Creates a new instance of `[`ListAssert`](ListAssert.html)` from the given [`Stream`](https://docs.oracle.com/javase/8/docs/api/java/util/stream/Stream.html).

    **Be aware that the `Stream` under test will be converted to a `List` when an assertion requires to inspect its content. Once this is done the `Stream` can't be reused as it has already been consumed.**

     Calling multiple methods on the returned [`ListAssert`](ListAssert.html) is safe as it only interacts with the [`List`](https://docs.oracle.com/javase/8/docs/api/java/util/List.html) built from the [`Stream`](https://docs.oracle.com/javase/8/docs/api/java/util/stream/Stream.html).

     Examples:

    ```
     // you can chain multiple assertions on the Stream as it is converted to a List
     assertThat(Stream.of(1, 2, 3)).contains(1)
                                   .doesNotContain(42);
    ```

     The following assertion fails as the Stream under test is converted to a List before being compared to the expected Stream:

    ```
     // FAIL: the Stream under test is converted to a List and compared to a Stream but a List is not a Stream.
     assertThat(Stream.of(1, 2, 3)).isEqualTo(Stream.of(1, 2, 3));
    ```

     These assertions succeed as `isEqualTo` and `isSameAs` checks references which does not require to convert the Stream to a List.

    ```
     // The following assertions succeed as it only performs reference checking which does not require to convert the Stream to a List
     Stream<Integer> stream = Stream.of(1, 2, 3);
     assertThat(stream).isEqualTo(stream)
                       .isSameAs(stream);
    ```

    Type Parameters:`ELEMENT` - the type of elements.Parameters:`actual` - the actual [`Stream`](https://docs.oracle.com/javase/8/docs/api/java/util/stream/Stream.html) value.Returns:the created assertion object.

  * ### assertThatStream ###

    public static \<ELEMENT\> [ListAssert](ListAssert.html)\<ELEMENT\> assertThatStream([Stream](https://docs.oracle.com/javase/8/docs/api/java/util/stream/Stream.html)\<? extends ELEMENT\> actual)

    Creates a new instance of `[`ListAssert`](ListAssert.html)` from the given [`Stream`](https://docs.oracle.com/javase/8/docs/api/java/util/stream/Stream.html).

     Use this over [`assertThat(Stream)`](#assertThat(java.util.stream.Stream)) in case of ambiguous method resolution when the object under test implements several interfaces Assertj provides `assertThat` for.

    **Be aware that the `Stream` under test will be converted to a `List` when an assertions require to inspect its content. Once this is done the `Stream` can't reused as it would have been consumed.**

     Calling multiple methods on the returned [`ListAssert`](ListAssert.html) is safe as it only interacts with the [`List`](https://docs.oracle.com/javase/8/docs/api/java/util/List.html) built from the [`Stream`](https://docs.oracle.com/javase/8/docs/api/java/util/stream/Stream.html).

     Examples:

    ```
     // you can chain multiple assertions on the Stream as it is converted to a List
     assertThat(Stream.of(1, 2, 3)).contains(1)
                                   .doesNotContain(42);
    ```

     The following assertion fails as the Stream under test is converted to a List before being compared to the expected Stream:

    ```
     // FAIL: the Stream under test is converted to a List and compared to a Stream but a List is not a Stream.
     assertThat(Stream.of(1, 2, 3)).isEqualTo(Stream.of(1, 2, 3));
    ```

     These assertions succeed as `isEqualTo` and `isSameAs` checks references which does not require to convert the Stream to a List.

    ```
     // The following assertions succeed as it only performs reference checking which does not require to convert the Stream to a List
     Stream<Integer> stream = Stream.of(1, 2, 3);
     assertThat(stream).isEqualTo(stream)
                       .isSameAs(stream);
    ```

    Type Parameters:`ELEMENT` - the type of elements.Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.23.0

  * ### assertThat ###

    public static [ListAssert](ListAssert.html)\<[Double](https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html)\> assertThat([DoubleStream](https://docs.oracle.com/javase/8/docs/api/java/util/stream/DoubleStream.html) actual)

    Creates a new instance of `[`ListAssert`](ListAssert.html)` from the given [`DoubleStream`](https://docs.oracle.com/javase/8/docs/api/java/util/stream/DoubleStream.html).

    **Be aware that the `DoubleStream` under test will be converted to a `List` when an assertion requires to inspect its content. Once this is done the `DoubleStream` can't reused as it has already been consumed.**

     Calling multiple methods on the returned [`ListAssert`](ListAssert.html) is safe as it only interacts with the [`List`](https://docs.oracle.com/javase/8/docs/api/java/util/List.html) built from the [`DoubleStream`](https://docs.oracle.com/javase/8/docs/api/java/util/stream/DoubleStream.html).

     Examples:

    ```
     // you can chain multiple assertions on the DoubleStream as it is converted to a List
     assertThat(DoubleStream.of(1.0, 2.0, 3.0)).contains(1.0)
                                               .doesNotContain(42.0);
    ```

     The following assertion fails as the DoubleStream under test is converted to a List before being compared to the expected DoubleStream:

    ```
     // FAIL: the DoubleStream under test is converted to a List and compared to a DoubleStream but a List is not a DoubleStream.
     assertThat(DoubleStream.of(1.0, 2.0, 3.0)).isEqualTo(DoubleStream.of(1.0, 2.0, 3.0));
    ```

     These assertions succeed as `isEqualTo` and `isSameAs` checks references which does not require to convert the DoubleStream to a List.

    ```
     // The following assertions succeed as it only performs reference checking which does not require to convert the DoubleStream to a List
     DoubleStream stream = DoubleStream.of(1.0, 2.0, 3.0);
     assertThat(stream).isEqualTo(stream)
                       .isSameAs(stream);
    ```

    Parameters:`actual` - the actual [`DoubleStream`](https://docs.oracle.com/javase/8/docs/api/java/util/stream/DoubleStream.html) value.Returns:the created assertion object.

  * ### assertThat ###

    public static [ListAssert](ListAssert.html)\<[Long](https://docs.oracle.com/javase/8/docs/api/java/lang/Long.html)\> assertThat([LongStream](https://docs.oracle.com/javase/8/docs/api/java/util/stream/LongStream.html) actual)

    Creates a new instance of `[`ListAssert`](ListAssert.html)` from the given [`LongStream`](https://docs.oracle.com/javase/8/docs/api/java/util/stream/LongStream.html).

    **Be aware that the `LongStream` under test will be converted to a `List` when an assertion requires to inspect its content. Once this is done the `LongStream` can't reused as it has already been consumed.**

     Calling multiple methods on the returned [`ListAssert`](ListAssert.html) is safe as it only interacts with the [`List`](https://docs.oracle.com/javase/8/docs/api/java/util/List.html) built from the [`LongStream`](https://docs.oracle.com/javase/8/docs/api/java/util/stream/LongStream.html).

     Examples:

    ```
     // you can chain multiple assertions on the LongStream as it is converted to a List
     assertThat(LongStream.of(1, 2, 3)).contains(1)
                                       .doesNotContain(42);
    ```

     The following assertion fails as the LongStream under test is converted to a List before being compared to the expected LongStream:

    ```
     // FAIL: the LongStream under test is converted to a List and compared to a LongStream but a List is not a LongStream.
     assertThat(LongStream.of(1, 2, 3)).isEqualTo(LongStream.of(1, 2, 3));
    ```

     These assertions succeed as `isEqualTo` and `isSameAs` checks references which does not require to convert the LongStream to a List.

    ```
     // The following assertions succeed as it only performs reference checking which does not require to convert the LongStream to a List
     LongStream stream = LongStream.of(1, 2, 3);
     assertThat(stream).isEqualTo(stream)
                       .isSameAs(stream);
    ```

    Parameters:`actual` - the actual [`LongStream`](https://docs.oracle.com/javase/8/docs/api/java/util/stream/LongStream.html) value.Returns:the created assertion object.

  * ### assertThat ###

    public static [ListAssert](ListAssert.html)\<[Integer](https://docs.oracle.com/javase/8/docs/api/java/lang/Integer.html)\> assertThat([IntStream](https://docs.oracle.com/javase/8/docs/api/java/util/stream/IntStream.html) actual)

    Creates a new instance of `[`ListAssert`](ListAssert.html)` from the given [`IntStream`](https://docs.oracle.com/javase/8/docs/api/java/util/stream/IntStream.html).

    **Be aware that the `IntStream` under test will be converted to a `List` when an assertion requires to inspect its content. Once this is done the `IntStream` can't reused as it has already been consumed.**

     Calling multiple methods on the returned [`ListAssert`](ListAssert.html) is safe as it only interacts with the [`List`](https://docs.oracle.com/javase/8/docs/api/java/util/List.html) built from the [`IntStream`](https://docs.oracle.com/javase/8/docs/api/java/util/stream/IntStream.html).

     Examples:

    ```
     // you can chain multiple assertions on the IntStream as it is converted to a List
     assertThat(IntStream.of(1, 2, 3)).contains(1)
                                      .doesNotContain(42);
    ```

     The following assertion fails as the IntStream under test is converted to a List before being compared to the expected IntStream:

    ```
     // FAIL: the IntStream under test is converted to a List and compared to a IntStream but a List is not a IntStream.
     assertThat(IntStream.of(1, 2, 3)).isEqualTo(IntStream.of(1, 2, 3));
    ```

     These assertions succeed as `isEqualTo` and `isSameAs` checks references which does not require to convert the IntStream to a List.

    ```
     // The following assertions succeed as it only performs reference checking which does not require to convert the IntStream to a List
     IntStream stream = IntStream.of(1, 2, 3);
     assertThat(stream).isEqualTo(stream)
                       .isSameAs(stream);
    ```

    Parameters:`actual` - the actual [`IntStream`](https://docs.oracle.com/javase/8/docs/api/java/util/stream/IntStream.html) value.Returns:the created assertion object.

  * ### assertThat ###

    public static \<ELEMENT\> [SpliteratorAssert](SpliteratorAssert.html)\<ELEMENT\> assertThat([Spliterator](https://docs.oracle.com/javase/8/docs/api/java/util/Spliterator.html)\<ELEMENT\> actual)

    Creates a new instance of `[`SpliteratorAssert`](SpliteratorAssert.html)` from the given [`Spliterator`](https://docs.oracle.com/javase/8/docs/api/java/util/Spliterator.html). Example:

    ```
     Spliterator<Integer> spliterator = Stream.of(1, 2, 3).spliterator();
     assertThat(spliterator).hasCharacteristics(Spliterator.SIZED);
    ```

    Type Parameters:`ELEMENT` - the type of elements.Parameters:`actual` - the spliterator to test.Returns:the created assertion object.Since:3.14.0

  * ### assertThat ###

    public static [AbstractPathAssert](AbstractPathAssert.html)\<?\> assertThat([Path](https://docs.oracle.com/javase/8/docs/api/java/nio/file/Path.html) actual)

    Creates a new instance of [`PathAssert`](PathAssert.html)

    Parameters:`actual` - the path to testReturns:the created assertion object

  * ### assertThatPath ###

    public static [AbstractPathAssert](AbstractPathAssert.html)\<?\> assertThatPath([Path](https://docs.oracle.com/javase/8/docs/api/java/nio/file/Path.html) actual)

    Creates a new instance of [`PathAssert`](PathAssert.html)

     Use this over [`assertThat(Path)`](#assertThat(java.nio.file.Path)) in case of ambiguous method resolution when the object under test implements several interfaces Assertj provides `assertThat` for.

    Parameters:`actual` - the path to testReturns:the created assertion objectSince:3.23.0

  * ### assertThat ###

    public static \<K,V\> [MapAssert](MapAssert.html)\<K,V\> assertThat([Map](https://docs.oracle.com/javase/8/docs/api/java/util/Map.html)\<K,V\> actual)

    Creates a new instance of `[`MapAssert`](MapAssert.html)`.

     Returned type is [`MapAssert`](MapAssert.html) as it overrides method to annotate them with [`SafeVarargs`](https://docs.oracle.com/javase/8/docs/api/java/lang/SafeVarargs.html) avoiding annoying warnings.

    Type Parameters:`K` - the type of keys in the map.`V` - the type of values in the map.Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThat ###

    public static \<T extends [Comparable](https://docs.oracle.com/javase/8/docs/api/java/lang/Comparable.html)\<? super T\>\>[AbstractComparableAssert](AbstractComparableAssert.html)\<?,T\> assertThat(T actual)

    Creates a new instance of `[`GenericComparableAssert`](GenericComparableAssert.html)` with standard comparison semantics.

    Type Parameters:`T` - the type of actual.Parameters:`actual` - the actual value.Returns:the created assertion object.

  * ### assertThatComparable ###

    public static \<T\>[AbstractUniversalComparableAssert](AbstractUniversalComparableAssert.html)\<?,T\> assertThatComparable([Comparable](https://docs.oracle.com/javase/8/docs/api/java/lang/Comparable.html)\<T\> actual)

    Creates a new instance of `[`UniversalComparableAssert`](UniversalComparableAssert.html)` with standard comparison semantics.

     Use this over [`assertThat(Comparable)`](#assertThat(T)) in case of ambiguous method resolution when the object under test implements several interfaces Assertj provides `assertThat` for.

    Type Parameters:`T` - the type of actual.Parameters:`actual` - the actual value.Returns:the created assertion object.Since:3.23.0

  * ### assertThat ###

    public static \<T extends [AssertDelegateTarget](AssertDelegateTarget.html)\> T assertThat(T assertion)

    Returns the given assertion. This method improves code readability by surrounding the given assertion with `assertThat`.

     Consider for example the following MyButton and MyButtonAssert classes:

    ```
     public class MyButton extends JButton {

       private boolean blinking;

       public boolean isBlinking() { return this.blinking; }

       public void setBlinking(boolean blink) { this.blinking = blink; }

     }

     private static class MyButtonAssert implements AssertDelegateTarget {

       private MyButton button;
       MyButtonAssert(MyButton button) { this.button = button; }

       void isBlinking() {
         // standard assertion from core Assertions.assertThat
         assertThat(button.isBlinking()).isTrue();
       }

       void isNotBlinking() {
         // standard assertion from core Assertions.assertThat
         assertThat(button.isBlinking()).isFalse();
       }
     }
    ```

     As MyButtonAssert implements AssertDelegateTarget, you can use `assertThat(buttonAssert).isBlinking();` instead of `buttonAssert.isBlinking();` to have easier to read assertions:

    ```
     @Test
     public void AssertDelegateTarget_example() {

       MyButton button = new MyButton();
       MyButtonAssert buttonAssert = new MyButtonAssert(button);

       // you can encapsulate MyButtonAssert assertions methods within assertThat
       assertThat(buttonAssert).isNotBlinking(); // same as : buttonAssert.isNotBlinking();

       button.setBlinking(true);

       assertThat(buttonAssert).isBlinking(); // same as : buttonAssert.isBlinking();
     }
    ```

    Type Parameters:`T` - the generic type of the user-defined assert.Parameters:`assertion` - the assertion to return.Returns:the given assertion.

  * ### useRepresentation ###

    public static void useRepresentation([Representation](../presentation/Representation.html) customRepresentation)

    Use the given [`Representation`](../presentation/Representation.html) in all remaining tests assertions.

    [`Representation`](../presentation/Representation.html) are used to format types in assertions error messages.

     An alternative way of using a different representation is to register one as a service, this approach is described in [`Representation`](../presentation/Representation.html), it requires more work than this method but has the advantage of not having to do anything in your tests and it would be applied to all the tests globally

     Example :

    ```
     private class Example {}

     private class CustomRepresentation extends StandardRepresentation {

       // override needed to hook specific formatting
       @Override
       public String toStringOf(Object o) {
         if (o instanceof Example) return "Example";
         // fallback to default formatting.
         return super.toStringOf(o);
       }

       // change String representation
       @Override
       protected String toStringOf(String s) {
         return "$" + s + "$";
       }
     }

     Assertions.useRepresentation(new CustomRepresentation());

     // this assertion fails ...
     assertThat(new Example()).isNull();
     // ... with error :
     // "expected:<[null]> but was:<[Example]>"

     // this one fails ...
     assertThat("foo").startsWith("bar");
     // ... with error :
     // Expecting:
     //   <$foo$>
     // to start with:
     //   <$bar$>
    ```

    Parameters:`customRepresentation` - the [`Representation`](../presentation/Representation.html) to useSince:2.5.0 / 3.5.0

  * ### registerFormatterForType ###

    public static \<T\> void registerFormatterForType([Class](https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html)\<T\> type, [Function](https://docs.oracle.com/javase/8/docs/api/java/util/function/Function.html)\<T,[String](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html)\> formatter)

    Assertions error messages uses a [`Representation`](../presentation/Representation.html) to format the different types involved, using this method you can control the formatting of a given type by providing a specific formatter.

     Registering a formatter makes it available for all AssertJ [`Representation`](../presentation/Representation.html):

    * [`StandardRepresentation`](../presentation/StandardRepresentation.html)
    * [`UnicodeRepresentation`](../presentation/UnicodeRepresentation.html)
    * [`HexadecimalRepresentation`](../presentation/HexadecimalRepresentation.html)
    * [`BinaryRepresentation`](../presentation/BinaryRepresentation.html)

     Example :

    ```
     // without specific formatter
     assertThat(STANDARD_REPRESENTATION.toStringOf(123L)).isEqualTo("123L");

     // register a formatter for Long
     Assertions.registerFormatterForType(Long.class, value -> "$" + value + "$");

     // now Long will be formatted between in $$ in error message.
     assertThat(STANDARD_REPRESENTATION.toStringOf(longNumber)).isEqualTo("$123$");

     // fails with error : expected:<$456$> but was:<$123$>
     assertThat(123L).isEqualTo(456L);
    ```

    Type Parameters:`T` - the type of format.Parameters:`type` - the class of the type to format`formatter` - the formatter [`Function`](https://docs.oracle.com/javase/8/docs/api/java/util/function/Function.html)Since:3.5.0

  * ### useDefaultRepresentation ###

    public static void useDefaultRepresentation()

    Reset the representaion being used to the one from the [`Configuration`](../configuration/Configuration.html) to revert the effect of calling [`useRepresentation(Representation)`](#useRepresentation(org.assertj.core.presentation.Representation)).

     Unless a specific [`Configuration`](../configuration/Configuration.html) is in use, this method resets the representation to the [`StandardRepresentation`](../presentation/StandardRepresentation.html).

    Since:2.5.0 / 3.5.0

---

Copyright © 2024 [AssertJ](https://assertj.github.io/doc/). All rights reserved. hljs.highlightAll();