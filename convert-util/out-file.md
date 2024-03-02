JUnit 5 User Guide
==========

Stefan Bechtold  
Sam Brannen  
Johannes Link  
Matthias Merdes  
Marc Philipp  
Juliette de Rancourt  
Christian Stein  
version 5.10.2

[](#overview)1. Overview
----------

The goal of this document is to provide comprehensive reference documentation for
programmers writing tests, extension authors, and engine authors as well as build tool
and IDE vendors.

This document is also available as a [PDF download](junit-user-guide-5.10.2.pdf).

### [](#overview-what-is-junit-5)1.1. What is JUnit 5? ###

Unlike previous versions of JUnit, JUnit 5 is composed of several different modules from
three different sub-projects.

**JUnit 5 = *JUnit Platform* + *JUnit Jupiter* + *JUnit Vintage***

The **JUnit Platform** serves as a foundation for [launching testing
frameworks](#launcher-api) on the JVM. It also defines the `[TestEngine](../api/org.junit.platform.engine/org/junit/platform/engine/TestEngine.html)` API for developing a testing
framework that runs on the platform. Furthermore, the platform provides a[Console Launcher](#running-tests-console-launcher) to launch the platform from the
command line and the [JUnit Platform Suite Engine](#junit-platform-suite-engine) for running a custom test suite using
one or more test engines on the platform. First-class support for the JUnit Platform also
exists in popular IDEs (see [IntelliJ IDEA](#running-tests-ide-intellij-idea),[Eclipse](#running-tests-ide-eclipse), [NetBeans](#running-tests-ide-netbeans), and[Visual Studio Code](#running-tests-ide-vscode)) and build tools (see [Gradle](#running-tests-build-gradle),[Maven](#running-tests-build-maven), and [Ant](#running-tests-build-ant)).

**JUnit Jupiter** is the combination of the [programming model](#writing-tests) and[extension model](#extensions) for writing tests and extensions in JUnit 5. The Jupiter
sub-project provides a `TestEngine` for running Jupiter based tests on the platform.

**JUnit Vintage** provides a `TestEngine` for running JUnit 3 and JUnit 4 based tests on
the platform. It requires JUnit 4.12 or later to be present on the class path or module
path.

### [](#overview-java-versions)1.2. Supported Java Versions ###

JUnit 5 requires Java 8 (or higher) at runtime. However, you can still test code that
has been compiled with previous versions of the JDK.

### [](#overview-getting-help)1.3. Getting Help ###

Ask JUnit 5 related questions on [Stack Overflow](https://stackoverflow.com/questions/tagged/junit5) or chat with the community on [Gitter](https://gitter.im/junit-team/junit5).

### [](#overview-getting-started)1.4. Getting Started ###

#### [](#overview-getting-started-junit-artifacts)1.4.1. Downloading JUnit Artifacts ####

To find out what artifacts are available for download and inclusion in your project, refer
to [Dependency Metadata](#dependency-metadata). To set up dependency management for your build, refer to[Build Support](#running-tests-build) and the [Example Projects](#overview-getting-started-example-projects).

#### [](#overview-getting-started-features)1.4.2. JUnit 5 Features ####

To find out what features are available in JUnit 5 and how to use them, read the
corresponding sections of this User Guide, organized by topic.

* [Writing Tests in JUnit Jupiter](#writing-tests)

* [Migrating from JUnit 4 to JUnit Jupiter](#migrating-from-junit4)

* [Running Tests](#running-tests)

* [Extension Model for JUnit Jupiter](#extensions)

* Advanced Topics

  * [JUnit Platform Launcher API](#launcher-api)

  * [JUnit Platform Test Kit](#testkit)

#### [](#overview-getting-started-example-projects)1.4.3. Example Projects ####

To see complete, working examples of projects that you can copy and experiment with, the[`junit5-samples`](https://github.com/junit-team/junit5-samples) repository is a good place to start. The`junit5-samples` repository hosts a collection of sample projects based on JUnit Jupiter,
JUnit Vintage, and other testing frameworks. You’ll find appropriate build scripts (e.g.,`build.gradle`, `pom.xml`, etc.) in the example projects. The links below highlight some
of the combinations you can choose from.

* For Gradle and Java, check out the `[junit5-jupiter-starter-gradle](https://github.com/junit-team/junit5-samples/tree/r5.10.2/junit5-jupiter-starter-gradle)` project.

* For Gradle and Kotlin, check out the `[junit5-jupiter-starter-gradle-kotlin](https://github.com/junit-team/junit5-samples/tree/r5.10.2/junit5-jupiter-starter-gradle-kotlin)` project.

* For Gradle and Groovy, check out the `[junit5-jupiter-starter-gradle-groovy](https://github.com/junit-team/junit5-samples/tree/r5.10.2/junit5-jupiter-starter-gradle-groovy)` project.

* For Maven, check out the `[junit5-jupiter-starter-maven](https://github.com/junit-team/junit5-samples/tree/r5.10.2/junit5-jupiter-starter-maven)` project.

* For Ant, check out the `[junit5-jupiter-starter-ant](https://github.com/junit-team/junit5-samples/tree/r5.10.2/junit5-jupiter-starter-ant)` project.

[](#writing-tests)2. Writing Tests
----------

The following example provides a glimpse at the minimum requirements for writing a test in
JUnit Jupiter. Subsequent sections of this chapter will provide further details on all
available features.

A first test case

```
import static org.junit.jupiter.api.Assertions.assertEquals;

import example.util.Calculator;

import org.junit.jupiter.api.Test;

class MyFirstJUnitJupiterTests {

    private final Calculator calculator = new Calculator();

    @Test
    void addition() {
        assertEquals(2, calculator.add(1, 1));
    }

}
```

### [](#writing-tests-annotations)2.1. Annotations ###

JUnit Jupiter supports the following annotations for configuring tests and extending the
framework.

Unless otherwise stated, all core annotations are located in the `[org.junit.jupiter.api](../api/org.junit.jupiter.api/org/junit/jupiter/api/package-summary.html)` package
in the `junit-jupiter-api` module.

|       Annotation       |                                                                                                                                                                                                                                                    Description                                                                                                                                                                                                                                                    |
|------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|        `@Test`         |                                                                                                                 Denotes that a method is a test method. Unlike JUnit 4’s `@Test` annotation, this annotation does not declare any attributes, since test extensions in JUnit Jupiter operate based on their own dedicated annotations. Such methods are *inherited* unless they are *overridden*.                                                                                                                 |
|  `@ParameterizedTest`  |                                                                                                                                                                                  Denotes that a method is a [parameterized test](#writing-tests-parameterized-tests). Such methods are *inherited* unless they are *overridden*.                                                                                                                                                                                  |
|    `@RepeatedTest`     |                                                                                                                                                                             Denotes that a method is a test template for a [repeated test](#writing-tests-repeated-tests). Such methods are *inherited* unless they are *overridden*.                                                                                                                                                                             |
|     `@TestFactory`     |                                                                                                                                                                               Denotes that a method is a test factory for [dynamic tests](#writing-tests-dynamic-tests). Such methods are *inherited* unless they are *overridden*.                                                                                                                                                                               |
|    `@TestTemplate`     |                                                                                                      Denotes that a method is a [template for test cases](#writing-tests-test-templates) designed to be invoked multiple times depending on the number of invocation contexts returned by the registered [providers](#extensions-test-templates). Such methods are *inherited* unless they are *overridden*.                                                                                                      |
|   `@TestClassOrder`    |                                                                                                                                                             Used to configure the [test class execution order](#writing-tests-test-execution-order-classes) for `@Nested` test classes in the annotated test class. Such annotations are *inherited*.                                                                                                                                                             |
|   `@TestMethodOrder`   |                                                                                                                                                     Used to configure the [test method execution order](#writing-tests-test-execution-order-methods) for the annotated test class; similar to JUnit 4’s `@FixMethodOrder`. Such annotations are *inherited*.                                                                                                                                                      |
|    `@TestInstance`     |                                                                                                                                                                              Used to configure the [test instance lifecycle](#writing-tests-test-instance-lifecycle) for the annotated test class. Such annotations are *inherited*.                                                                                                                                                                              |
|     `@DisplayName`     |                                                                                                                                                                                      Declares a custom [display name](#writing-tests-display-names) for the test class or test method. Such annotations are not *inherited*.                                                                                                                                                                                      |
|`@DisplayNameGeneration`|                                                                                                                                                                                      Declares a custom [display name generator](#writing-tests-display-name-generator) for the test class. Such annotations are *inherited*.                                                                                                                                                                                      |
|     `@BeforeEach`      |                                                                      Denotes that the annotated method should be executed *before* **each** `@Test`, `@RepeatedTest`, `@ParameterizedTest`, or `@TestFactory` method in the current class; analogous to JUnit 4’s `@Before`. Such methods are *inherited* – unless they are *overridden* or *superseded* (i.e., replaced based on signature only, irrespective of Java’s visibility rules).                                                                       |
|      `@AfterEach`      |                                                                       Denotes that the annotated method should be executed *after* **each** `@Test`, `@RepeatedTest`, `@ParameterizedTest`, or `@TestFactory` method in the current class; analogous to JUnit 4’s `@After`. Such methods are *inherited* – unless they are *overridden* or *superseded* (i.e., replaced based on signature only, irrespective of Java’s visibility rules).                                                                        |
|      `@BeforeAll`      | Denotes that the annotated method should be executed *before* **all** `@Test`, `@RepeatedTest`, `@ParameterizedTest`, and `@TestFactory` methods in the current class; analogous to JUnit 4’s `@BeforeClass`. Such methods are *inherited* – unless they are *hidden*, *overridden*, or *superseded*, (i.e., replaced based on signature only, irrespective of Java’s visibility rules) – and must be `static` unless the "per-class" [test instance lifecycle](#writing-tests-test-instance-lifecycle) is used.  |
|      `@AfterAll`       |  Denotes that the annotated method should be executed *after* **all** `@Test`, `@RepeatedTest`, `@ParameterizedTest`, and `@TestFactory` methods in the current class; analogous to JUnit 4’s `@AfterClass`. Such methods are *inherited* – unless they are *hidden*, *overridden*, or *superseded*, (i.e., replaced based on signature only, irrespective of Java’s visibility rules) – and must be `static` unless the "per-class" [test instance lifecycle](#writing-tests-test-instance-lifecycle) is used.   |
|       `@Nested`        |Denotes that the annotated class is a non-static [nested test class](#writing-tests-nested). On Java 8 through Java 15, `@BeforeAll` and `@AfterAll` methods cannot be used directly in a `@Nested` test class unless the "per-class" [test instance lifecycle](#writing-tests-test-instance-lifecycle) is used. Beginning with Java 16, `@BeforeAll` and `@AfterAll` methods can be declared as `static` in a `@Nested` test class with either test instance lifecycle mode. Such annotations are not *inherited*.|
|         `@Tag`         |                                                                                                                       Used to declare [tags for filtering tests](#writing-tests-tagging-and-filtering), either at the class or method level; analogous to test groups in TestNG or Categories in JUnit 4. Such annotations are *inherited* at the class level but not at the method level.                                                                                                                        |
|      `@Disabled`       |                                                                                                                                                                                 Used to [disable](#writing-tests-disabling) a test class or test method; analogous to JUnit 4’s `@Ignore`. Such annotations are not *inherited*.                                                                                                                                                                                  |
|       `@Timeout`       |                                                                                                                                                                                Used to fail a test, test factory, test template, or lifecycle method if its execution exceeds a given duration. Such annotations are *inherited*.                                                                                                                                                                                 |
|     `@ExtendWith`      |                                                                                                                                                                                               Used to [register extensions declaratively](#extensions-registration-declarative). Such annotations are *inherited*.                                                                                                                                                                                                |
|  `@RegisterExtension`  |                                                                                                                                                                             Used to [register extensions programmatically](#extensions-registration-programmatic) via fields. Such fields are *inherited* unless they are *shadowed*.                                                                                                                                                                             |
|       `@TempDir`       |                                                                                                                                           Used to supply a [temporary directory](#writing-tests-built-in-extensions-TempDirectory) via field injection or parameter injection in a lifecycle method or test method; located in the `org.junit.jupiter.api.io` package.                                                                                                                                            |

|   |Some annotations may currently be *experimental*. Consult the table in[Experimental APIs](#api-evolution-experimental-apis) for details.|
|---|----------------------------------------------------------------------------------------------------------------------------------------|

#### [](#writing-tests-meta-annotations)2.1.1. Meta-Annotations and Composed Annotations ####

JUnit Jupiter annotations can be used as *meta-annotations*. That means that you can
define your own *composed annotation* that will automatically *inherit* the semantics of
its meta-annotations.

For example, instead of copying and pasting `@Tag("fast")` throughout your code base (see[Tagging and Filtering](#writing-tests-tagging-and-filtering)), you can create a custom *composed annotation*named `@Fast` as follows. `@Fast` can then be used as a drop-in replacement for`@Tag("fast")`.

```
import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

import org.junit.jupiter.api.Tag;

@Target({ ElementType.TYPE, ElementType.METHOD })
@Retention(RetentionPolicy.RUNTIME)
@Tag("fast")
public @interface Fast {
}
```

The following `@Test` method demonstrates usage of the `@Fast` annotation.

```
@Fast
@Test
void myFastTest() {
    // ...
}
```

You can even take that one step further by introducing a custom `@FastTest` annotation
that can be used as a drop-in replacement for `@Tag("fast")` *and* `@Test`.

```
import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

@Target(ElementType.METHOD)
@Retention(RetentionPolicy.RUNTIME)
@Tag("fast")
@Test
public @interface FastTest {
}
```

JUnit automatically recognizes the following as a `@Test` method that is tagged with
"fast".

```
@FastTest
void myFastTest() {
    // ...
}
```

### [](#writing-tests-definitions)2.2. Definitions ###

Platform Concepts

Container

a node in the test tree that contains other containers or tests as its children (e.g. a *test class*).

Test

a node in the test tree that verifies expected behavior when executed (e.g. a `@Test` method).

Jupiter Concepts

Lifecycle Method

any method that is directly annotated or meta-annotated with`@BeforeAll`, `@AfterAll`, `@BeforeEach`, or `@AfterEach`.

Test Class

any top-level class, `static` member class, or [`@Nested` class](#writing-tests-nested) that contains at least one *test method*, i.e. a *container*.
Test classes must not be `abstract` and must have a single constructor.

Test Method

any instance method that is directly annotated or meta-annotated with`@Test`, `@RepeatedTest`, `@ParameterizedTest`, `@TestFactory`, or `@TestTemplate`.
With the exception of `@Test`, these create a *container* in the test tree that groups*tests* or, potentially (for `@TestFactory`), other *containers*.

### [](#writing-tests-classes-and-methods)2.3. Test Classes and Methods ###

Test methods and lifecycle methods may be declared locally within the current test class,
inherited from superclasses, or inherited from interfaces (see[Test Interfaces and Default Methods](#writing-tests-test-interfaces-and-default-methods)). In addition, test methods and
lifecycle methods must not be `abstract` and must not return a value (except `@TestFactory`methods which are required to return a value).

|   |Class and method visibility<br/><br/>Test classes, test methods, and lifecycle methods are not required to be `public`, but<br/>they must *not* be `private`.<br/><br/>It is generally recommended to omit the `public` modifier for test classes, test methods,<br/>and lifecycle methods unless there is a technical reason for doing so – for example, when<br/>a test class is extended by a test class in another package. Another technical reason for<br/>making classes and methods `public` is to simplify testing on the module path when using<br/>the Java Module System.|
|---|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

The following test class demonstrates the use of `@Test` methods and all supported
lifecycle methods. For further information on runtime semantics, see[Test Execution Order](#writing-tests-test-execution-order) and[Wrapping Behavior of Callbacks](#extensions-execution-order-wrapping-behavior).

A standard test class

```
import static org.junit.jupiter.api.Assertions.fail;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Disabled;
import org.junit.jupiter.api.Test;

class StandardTests {

    @BeforeAll
    static void initAll() {
    }

    @BeforeEach
    void init() {
    }

    @Test
    void succeedingTest() {
    }

    @Test
    void failingTest() {
        fail("a failing test");
    }

    @Test
    @Disabled("for demonstration purposes")
    void skippedTest() {
        // not executed
    }

    @Test
    void abortedTest() {
        assumeTrue("abc".contains("Z"));
        fail("test should have been aborted");
    }

    @AfterEach
    void tearDown() {
    }

    @AfterAll
    static void tearDownAll() {
    }

}
```

### [](#writing-tests-display-names)2.4. Display Names ###

Test classes and test methods can declare custom display names via `@DisplayName` — with
spaces, special characters, and even emojis — that will be displayed in test reports and
by test runners and IDEs.

```
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

@DisplayName("A special test case")
class DisplayNameDemo {

    @Test
    @DisplayName("Custom test name containing spaces")
    void testWithDisplayNameContainingSpaces() {
    }

    @Test
    @DisplayName("╯°□°）╯")
    void testWithDisplayNameContainingSpecialCharacters() {
    }

    @Test
    @DisplayName("😱")
    void testWithDisplayNameContainingEmoji() {
    }

}
```

#### [](#writing-tests-display-name-generator)2.4.1. Display Name Generators ####

JUnit Jupiter supports custom display name generators that can be configured via the`@DisplayNameGeneration` annotation. Values provided via `@DisplayName` annotations
always take precedence over display names generated by a `DisplayNameGenerator`.

Generators can be created by implementing `DisplayNameGenerator`. Here are some default
ones available in Jupiter:

|DisplayNameGenerator |                                              Behavior                                              |
|---------------------|----------------------------------------------------------------------------------------------------|
|     `Standard`      |Matches the standard display name generation behavior in place since JUnit Jupiter 5.0 was released.|
|      `Simple`       |                    Removes trailing parentheses for methods with no parameters.                    |
|`ReplaceUnderscores` |                                 Replaces underscores with spaces.                                  |
|`IndicativeSentences`|   Generates complete sentences by concatenating the names of the test and the enclosing classes.   |

Note that for `IndicativeSentences`, you can customize the separator and the
underlying generator by using `@IndicativeSentencesGeneration` as shown in the
following example.

```
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.DisplayNameGeneration;
import org.junit.jupiter.api.DisplayNameGenerator;
import org.junit.jupiter.api.DisplayNameGenerator.ReplaceUnderscores;
import org.junit.jupiter.api.IndicativeSentencesGeneration;
import org.junit.jupiter.api.Nested;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

class DisplayNameGeneratorDemo {

    @Nested
    @DisplayNameGeneration(DisplayNameGenerator.ReplaceUnderscores.class)
    class A_year_is_not_supported {

        @Test
        void if_it_is_zero() {
        }

        @DisplayName("A negative value for year is not supported by the leap year computation.")
        @ParameterizedTest(name = "For example, year {0} is not supported.")
        @ValueSource(ints = { -1, -4 })
        void if_it_is_negative(int year) {
        }

    }

    @Nested
    @IndicativeSentencesGeneration(separator = " -> ", generator = ReplaceUnderscores.class)
    class A_year_is_a_leap_year {

        @Test
        void if_it_is_divisible_by_4_but_not_by_100() {
        }

        @ParameterizedTest(name = "Year {0} is a leap year.")
        @ValueSource(ints = { 2016, 2020, 2048 })
        void if_it_is_one_of_the_following_years(int year) {
        }

    }

}
```

```
+-- DisplayNameGeneratorDemo [OK]
  +-- A year is not supported [OK]
  | +-- A negative value for year is not supported by the leap year computation. [OK]
  | | +-- For example, year -1 is not supported. [OK]
  | | '-- For example, year -4 is not supported. [OK]
  | '-- if it is zero() [OK]
  '-- A year is a leap year [OK]
    +-- A year is a leap year -> if it is divisible by 4 but not by 100. [OK]
    '-- A year is a leap year -> if it is one of the following years. [OK]
      +-- Year 2016 is a leap year. [OK]
      +-- Year 2020 is a leap year. [OK]
      '-- Year 2048 is a leap year. [OK]
```

#### [](#writing-tests-display-name-generator-default)2.4.2. Setting the Default Display Name Generator ####

You can use the `junit.jupiter.displayname.generator.default`[configuration parameter](#running-tests-config-params) to specify the fully qualified
class name of the `DisplayNameGenerator` you would like to use by default. Just like for
display name generators configured via the `@DisplayNameGeneration` annotation, the
supplied class has to implement the `DisplayNameGenerator` interface. The default display
name generator will be used for all tests unless the `@DisplayNameGeneration` annotation
is present on an enclosing test class or test interface. Values provided via`@DisplayName` annotations always take precedence over display names generated by a`DisplayNameGenerator`.

For example, to use the `ReplaceUnderscores` display name generator by default, you should
set the configuration parameter to the corresponding fully qualified class name (e.g., in`src/test/resources/junit-platform.properties`):

```
junit.jupiter.displayname.generator.default = \
    org.junit.jupiter.api.DisplayNameGenerator$ReplaceUnderscores
```

Similarly, you can specify the fully qualified name of any custom class that implements`DisplayNameGenerator`.

In summary, the display name for a test class or method is determined according to the
following precedence rules:

1. value of the `@DisplayName` annotation, if present

2. by calling the `DisplayNameGenerator` specified in the `@DisplayNameGeneration`annotation, if present

3. by calling the default `DisplayNameGenerator` configured via the configuration
   parameter, if present

4. by calling `org.junit.jupiter.api.DisplayNameGenerator.Standard`

### [](#writing-tests-assertions)2.5. Assertions ###

JUnit Jupiter comes with many of the assertion methods that JUnit 4 has and adds a few
that lend themselves well to being used with Java 8 lambdas. All JUnit Jupiter assertions
are `static` methods in the `[org.junit.jupiter.api.Assertions](../api/org.junit.jupiter.api/org/junit/jupiter/api/Assertions.html)` class.

```
import static java.time.Duration.ofMillis;
import static java.time.Duration.ofMinutes;
import static org.junit.jupiter.api.Assertions.assertAll;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTimeout;
import static org.junit.jupiter.api.Assertions.assertTimeoutPreemptively;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.concurrent.CountDownLatch;

import example.domain.Person;
import example.util.Calculator;

import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

class AssertionsDemo {

    private final Calculator calculator = new Calculator();

    private final Person person = new Person("Jane", "Doe");

    @Test
    void standardAssertions() {
        assertEquals(2, calculator.add(1, 1));
        assertEquals(4, calculator.multiply(2, 2),
                "The optional failure message is now the last parameter");
        assertTrue('a' < 'b', () -> "Assertion messages can be lazily evaluated -- "
                + "to avoid constructing complex messages unnecessarily.");
    }

    @Test
    void groupedAssertions() {
        // In a grouped assertion all assertions are executed, and all
        // failures will be reported together.
        assertAll("person",
            () -> assertEquals("Jane", person.getFirstName()),
            () -> assertEquals("Doe", person.getLastName())
        );
    }

    @Test
    void dependentAssertions() {
        // Within a code block, if an assertion fails the
        // subsequent code in the same block will be skipped.
        assertAll("properties",
            () -> {
                String firstName = person.getFirstName();
                assertNotNull(firstName);

                // Executed only if the previous assertion is valid.
                assertAll("first name",
                    () -> assertTrue(firstName.startsWith("J")),
                    () -> assertTrue(firstName.endsWith("e"))
                );
            },
            () -> {
                // Grouped assertion, so processed independently
                // of results of first name assertions.
                String lastName = person.getLastName();
                assertNotNull(lastName);

                // Executed only if the previous assertion is valid.
                assertAll("last name",
                    () -> assertTrue(lastName.startsWith("D")),
                    () -> assertTrue(lastName.endsWith("e"))
                );
            }
        );
    }

    @Test
    void exceptionTesting() {
        Exception exception = assertThrows(ArithmeticException.class, () ->
            calculator.divide(1, 0));
        assertEquals("/ by zero", exception.getMessage());
    }

    @Test
    void timeoutNotExceeded() {
        // The following assertion succeeds.
        assertTimeout(ofMinutes(2), () -> {
            // Perform task that takes less than 2 minutes.
        });
    }

    @Test
    void timeoutNotExceededWithResult() {
        // The following assertion succeeds, and returns the supplied object.
        String actualResult = assertTimeout(ofMinutes(2), () -> {
            return "a result";
        });
        assertEquals("a result", actualResult);
    }

    @Test
    void timeoutNotExceededWithMethod() {
        // The following assertion invokes a method reference and returns an object.
        String actualGreeting = assertTimeout(ofMinutes(2), AssertionsDemo::greeting);
        assertEquals("Hello, World!", actualGreeting);
    }

    @Test
    void timeoutExceeded() {
        // The following assertion fails with an error message similar to:
        // execution exceeded timeout of 10 ms by 91 ms
        assertTimeout(ofMillis(10), () -> {
            // Simulate task that takes more than 10 ms.
            Thread.sleep(100);
        });
    }

    @Test
    void timeoutExceededWithPreemptiveTermination() {
        // The following assertion fails with an error message similar to:
        // execution timed out after 10 ms
        assertTimeoutPreemptively(ofMillis(10), () -> {
            // Simulate task that takes more than 10 ms.
            new CountDownLatch(1).await();
        });
    }

    private static String greeting() {
        return "Hello, World!";
    }

}
```

|   |Preemptive Timeouts with `assertTimeoutPreemptively()`<br/><br/>The various `assertTimeoutPreemptively()` methods in the `Assertions` class execute<br/>the provided `executable` or `supplier` in a different thread than that of the calling<br/>code. This behavior can lead to undesirable side effects if the code that is executed<br/>within the `executable` or `supplier` relies on `java.lang.ThreadLocal` storage.<br/><br/>One common example of this is the transactional testing support in the Spring Framework.<br/>Specifically, Spring’s testing support binds transaction state to the current thread (via<br/>a `ThreadLocal`) before a test method is invoked. Consequently, if an `executable` or`supplier` provided to `assertTimeoutPreemptively()` invokes Spring-managed components<br/>that participate in transactions, any actions taken by those components will not be rolled<br/>back with the test-managed transaction. On the contrary, such actions will be committed to<br/>the persistent store (e.g., relational database) even though the test-managed transaction<br/>is rolled back.<br/><br/>Similar side effects may be encountered with other frameworks that rely on`ThreadLocal` storage.|
|---|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

#### [](#writing-tests-assertions-kotlin)2.5.1. Kotlin Assertion Support ####

JUnit Jupiter also comes with a few assertion methods that lend themselves well to being
used in [Kotlin](https://kotlinlang.org/). All JUnit Jupiter Kotlin assertions are top-level
functions in the `org.junit.jupiter.api` package.

```
import example.domain.Person
import example.util.Calculator
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Tag
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertAll
import org.junit.jupiter.api.assertDoesNotThrow
import org.junit.jupiter.api.assertThrows
import org.junit.jupiter.api.assertTimeout
import org.junit.jupiter.api.assertTimeoutPreemptively
import java.time.Duration

class KotlinAssertionsDemo {

    private val person = Person("Jane", "Doe")
    private val people = setOf(person, Person("John", "Doe"))

    @Test
    fun `exception absence testing`() {
        val calculator = Calculator()
        val result = assertDoesNotThrow("Should not throw an exception") {
            calculator.divide(0, 1)
        }
        assertEquals(0, result)
    }

    @Test
    fun `expected exception testing`() {
        val calculator = Calculator()
        val exception = assertThrows<ArithmeticException> ("Should throw an exception") {
            calculator.divide(1, 0)
        }
        assertEquals("/ by zero", exception.message)
    }

    @Test
    fun `grouped assertions`() {
        assertAll(
            "Person properties",
            { assertEquals("Jane", person.firstName) },
            { assertEquals("Doe", person.lastName) }
        )
    }

    @Test
    fun `grouped assertions from a stream`() {
        assertAll(
            "People with first name starting with J",
            people
                .stream()
                .map {
                    // This mapping returns Stream<() -> Unit>
                    { assertTrue(it.firstName.startsWith("J")) }
                }
        )
    }

    @Test
    fun `grouped assertions from a collection`() {
        assertAll(
            "People with last name of Doe",
            people.map { { assertEquals("Doe", it.lastName) } }
        )
    }

    @Test
    fun `timeout not exceeded testing`() {
        val fibonacciCalculator = FibonacciCalculator()
        val result = assertTimeout(Duration.ofMillis(1000)) {
            fibonacciCalculator.fib(14)
        }
        assertEquals(377, result)
    }

    @Test
    fun `timeout exceeded with preemptive termination`() {
        // The following assertion fails with an error message similar to:
        // execution timed out after 10 ms
        assertTimeoutPreemptively(Duration.ofMillis(10)) {
            // Simulate task that takes more than 10 ms.
            Thread.sleep(100)
        }
    }
}
```

#### [](#writing-tests-assertions-third-party)2.5.2. Third-party Assertion Libraries ####

Even though the assertion facilities provided by JUnit Jupiter are sufficient for many
testing scenarios, there are times when more power and additional functionality such as*matchers* are desired or required. In such cases, the JUnit team recommends the use of
third-party assertion libraries such as [AssertJ](https://assertj.github.io/doc/), [Hamcrest](https://hamcrest.org/JavaHamcrest/), [Truth](https://truth.dev/), etc. Developers
are therefore free to use the assertion library of their choice.

For example, the combination of *matchers* and a fluent API can be used to make
assertions more descriptive and readable. However, JUnit Jupiter’s `[org.junit.jupiter.api.Assertions](../api/org.junit.jupiter.api/org/junit/jupiter/api/Assertions.html)` class
does not provide an[`assertThat()`](https://junit.org/junit4/javadoc/latest/org/junit/Assert.html#assertThat)method like the one found in JUnit 4’s `org.junit.Assert` class which accepts a Hamcrest[`Matcher`](https://junit.org/junit4/javadoc/latest/org/hamcrest/Matcher.html). Instead,
developers are encouraged to use the built-in support for matchers provided by third-party
assertion libraries.

The following example demonstrates how to use the `assertThat()` support from Hamcrest in
a JUnit Jupiter test. As long as the Hamcrest library has been added to the classpath,
you can statically import methods such as `assertThat()`, `is()`, and `equalTo()` and
then use them in tests like in the `assertWithHamcrestMatcher()` method below.

```
import static org.hamcrest.CoreMatchers.equalTo;
import static org.hamcrest.CoreMatchers.is;
import static org.hamcrest.MatcherAssert.assertThat;

import example.util.Calculator;

import org.junit.jupiter.api.Test;

class HamcrestAssertionsDemo {

    private final Calculator calculator = new Calculator();

    @Test
    void assertWithHamcrestMatcher() {
        assertThat(calculator.subtract(4, 1), is(equalTo(3)));
    }

}
```

Naturally, legacy tests based on the JUnit 4 programming model can continue using`org.junit.Assert#assertThat`.

### [](#writing-tests-assumptions)2.6. Assumptions ###

JUnit Jupiter comes with a subset of the assumption methods that JUnit 4 provides and
adds a few that lend themselves well to being used with Java 8 lambda expressions and
method references. All JUnit Jupiter assumptions are static methods in the`[org.junit.jupiter.api.Assumptions](../api/org.junit.jupiter.api/org/junit/jupiter/api/Assumptions.html)` class.

```
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assumptions.assumeTrue;
import static org.junit.jupiter.api.Assumptions.assumingThat;

import example.util.Calculator;

import org.junit.jupiter.api.Test;

class AssumptionsDemo {

    private final Calculator calculator = new Calculator();

    @Test
    void testOnlyOnCiServer() {
        assumeTrue("CI".equals(System.getenv("ENV")));
        // remainder of test
    }

    @Test
    void testOnlyOnDeveloperWorkstation() {
        assumeTrue("DEV".equals(System.getenv("ENV")),
            () -> "Aborting test: not on developer workstation");
        // remainder of test
    }

    @Test
    void testInAllEnvironments() {
        assumingThat("CI".equals(System.getenv("ENV")),
            () -> {
                // perform these assertions only on the CI server
                assertEquals(2, calculator.divide(4, 2));
            });

        // perform these assertions in all environments
        assertEquals(42, calculator.multiply(6, 7));
    }

}
```

|   |As of JUnit Jupiter 5.4, it is also possible to use methods from JUnit 4’s`org.junit.Assume` class for assumptions. Specifically, JUnit Jupiter supports JUnit 4’s`AssumptionViolatedException` to signal that a test should be aborted instead of marked<br/>as a failure.|
|---|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

### [](#writing-tests-disabling)2.7. Disabling Tests ###

Entire test classes or individual test methods may be *disabled* via the `[@Disabled](../api/org.junit.jupiter.api/org/junit/jupiter/api/Disabled.html)`annotation, via one of the annotations discussed in[Conditional Test Execution](#writing-tests-conditional-execution), or via a custom [`ExecutionCondition`](#extensions-conditions).

Here’s a `@Disabled` test class.

```
import org.junit.jupiter.api.Disabled;
import org.junit.jupiter.api.Test;

@Disabled("Disabled until bug #99 has been fixed")
class DisabledClassDemo {

    @Test
    void testWillBeSkipped() {
    }

}
```

And here’s a test class that contains a `@Disabled` test method.

```
import org.junit.jupiter.api.Disabled;
import org.junit.jupiter.api.Test;

class DisabledTestsDemo {

    @Disabled("Disabled until bug #42 has been resolved")
    @Test
    void testWillBeSkipped() {
    }

    @Test
    void testWillBeExecuted() {
    }

}
```

|   |`@Disabled` may be declared without providing a *reason*; however, the JUnit team<br/>recommends that developers provide a short explanation for why a test class or test<br/>method has been disabled. Consequently, the above examples both show the use of a reason — for example, `@Disabled("Disabled until bug #42 has been resolved")`. Some development<br/>teams even require the presence of issue tracking numbers in the *reason* for automated<br/>traceability, etc.|
|---|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

|   |`@Disabled` is not `@Inherited`. Consequently, if you wish to disable a class whose<br/>superclass is `@Disabled`, you must redeclare `@Disabled` on the subclass.|
|---|------------------------------------------------------------------------------------------------------------------------------------------------------------------|

### [](#writing-tests-conditional-execution)2.8. Conditional Test Execution ###

The [`ExecutionCondition`](#extensions-conditions) extension API in JUnit Jupiter allows
developers to either *enable* or *disable* a container or test based on certain
conditions *programmatically*. The simplest example of such a condition is the built-in`[DisabledCondition](https://github.com/junit-team/junit5/tree/r5.10.2/junit-jupiter-engine/src/main/java/org/junit/jupiter/engine/extension/DisabledCondition.java)` which supports the `[@Disabled](../api/org.junit.jupiter.api/org/junit/jupiter/api/Disabled.html)` annotation (see[Disabling Tests](#writing-tests-disabling)). In addition to `@Disabled`, JUnit Jupiter also supports
several other annotation-based conditions in the `org.junit.jupiter.api.condition`package that allow developers to enable or disable containers and tests *declaratively*.
When multiple `ExecutionCondition` extensions are registered, a container or test is
disabled as soon as one of the conditions returns *disabled*. If you wish to provide
details about why they might be disabled, every annotation associated with these built-in
conditions has a `disabledReason` attribute available for that purpose.

See [`ExecutionCondition`](#extensions-conditions) and the following sections for
details.

|   |Composed Annotations<br/><br/>Note that any of the *conditional* annotations listed in the following sections may also<br/>be used as a meta-annotation in order to create a custom *composed annotation*. For<br/>example, the `@TestOnMac` annotation in the[@EnabledOnOs demo](#writing-tests-conditional-execution-os-demo) shows how you can<br/>combine `@Test` and `@EnabledOnOs` in a single, reusable annotation.|
|---|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

|   |*Conditional* annotations in JUnit Jupiter are not `@Inherited`. Consequently, if you wish<br/>to apply the same semantics to subclasses, each conditional annotation must be redeclared<br/>on each subclass.|
|---|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

|   |Unless otherwise stated, each of the *conditional* annotations listed in the following<br/>sections can only be declared once on a given test interface, test class, or test method.<br/>If a conditional annotation is directly present, indirectly present, or meta-present<br/>multiple times on a given element, only the first such annotation discovered by JUnit will<br/>be used; any additional declarations will be silently ignored. Note, however, that each<br/>conditional annotation may be used in conjunction with other conditional annotations in<br/>the `org.junit.jupiter.api.condition` package.|
|---|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

#### [](#writing-tests-conditional-execution-os)2.8.1. Operating System and Architecture Conditions ####

A container or test may be enabled or disabled on a particular operating system,
architecture, or combination of both via the `[@EnabledOnOs](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/EnabledOnOs.html)` and `[@DisabledOnOs](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/DisabledOnOs.html)`annotations.

Conditional execution based on operating system

```
@Test
@EnabledOnOs(MAC)
void onlyOnMacOs() {
    // ...
}

@TestOnMac
void testOnMac() {
    // ...
}

@Test
@EnabledOnOs({ LINUX, MAC })
void onLinuxOrMac() {
    // ...
}

@Test
@DisabledOnOs(WINDOWS)
void notOnWindows() {
    // ...
}

@Target(ElementType.METHOD)
@Retention(RetentionPolicy.RUNTIME)
@Test
@EnabledOnOs(MAC)
@interface TestOnMac {
}
```

Conditional execution based on architecture

```
@Test
@EnabledOnOs(architectures = "aarch64")
void onAarch64() {
    // ...
}

@Test
@DisabledOnOs(architectures = "x86_64")
void notOnX86_64() {
    // ...
}

@Test
@EnabledOnOs(value = MAC, architectures = "aarch64")
void onNewMacs() {
    // ...
}

@Test
@DisabledOnOs(value = MAC, architectures = "aarch64")
void notOnNewMacs() {
    // ...
}
```

#### [](#writing-tests-conditional-execution-jre)2.8.2. Java Runtime Environment Conditions ####

A container or test may be enabled or disabled on particular versions of the Java
Runtime Environment (JRE) via the `[@EnabledOnJre](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/EnabledOnJre.html)` and `[@DisabledOnJre](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/DisabledOnJre.html)` annotations
or on a particular range of versions of the JRE via the `[@EnabledForJreRange](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/EnabledForJreRange.html)` and`[@DisabledForJreRange](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/DisabledForJreRange.html)` annotations. The range defaults to `[JRE](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/JRE.html).JAVA_8` as the lower
border (`min`) and `[JRE](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/JRE.html).OTHER` as the higher border (`max`), which allows usage of
half open ranges.

```
@Test
@EnabledOnJre(JAVA_8)
void onlyOnJava8() {
    // ...
}

@Test
@EnabledOnJre({ JAVA_9, JAVA_10 })
void onJava9Or10() {
    // ...
}

@Test
@EnabledForJreRange(min = JAVA_9, max = JAVA_11)
void fromJava9to11() {
    // ...
}

@Test
@EnabledForJreRange(min = JAVA_9)
void fromJava9toCurrentJavaFeatureNumber() {
    // ...
}

@Test
@EnabledForJreRange(max = JAVA_11)
void fromJava8To11() {
    // ...
}

@Test
@DisabledOnJre(JAVA_9)
void notOnJava9() {
    // ...
}

@Test
@DisabledForJreRange(min = JAVA_9, max = JAVA_11)
void notFromJava9to11() {
    // ...
}

@Test
@DisabledForJreRange(min = JAVA_9)
void notFromJava9toCurrentJavaFeatureNumber() {
    // ...
}

@Test
@DisabledForJreRange(max = JAVA_11)
void notFromJava8to11() {
    // ...
}
```

#### [](#writing-tests-conditional-execution-native)2.8.3. Native Image Conditions ####

A container or test may be enabled or disabled within a[GraalVM native image](https://www.graalvm.org/reference-manual/native-image/) via the`[@EnabledInNativeImage](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/EnabledInNativeImage.html)` and `[@DisabledInNativeImage](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/DisabledInNativeImage.html)` annotations. These annotations are
typically used when running tests within a native image using the Gradle and Maven
plug-ins from the GraalVM [Native
Build Tools](https://graalvm.github.io/native-build-tools/latest/) project.

```
@Test
@EnabledInNativeImage
void onlyWithinNativeImage() {
    // ...
}

@Test
@DisabledInNativeImage
void neverWithinNativeImage() {
    // ...
}
```

#### [](#writing-tests-conditional-execution-system-properties)2.8.4. System Property Conditions ####

A container or test may be enabled or disabled based on the value of the `named` JVM
system property via the `[@EnabledIfSystemProperty](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/EnabledIfSystemProperty.html)` and `[@DisabledIfSystemProperty](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/DisabledIfSystemProperty.html)`annotations. The value supplied via the `matches` attribute will be interpreted as a
regular expression.

```
@Test
@EnabledIfSystemProperty(named = "os.arch", matches = ".*64.*")
void onlyOn64BitArchitectures() {
    // ...
}

@Test
@DisabledIfSystemProperty(named = "ci-server", matches = "true")
void notOnCiServer() {
    // ...
}
```

|   |As of JUnit Jupiter 5.6, `[@EnabledIfSystemProperty](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/EnabledIfSystemProperty.html)` and `[@DisabledIfSystemProperty](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/DisabledIfSystemProperty.html)` are*repeatable annotations*. Consequently, these annotations may be declared multiple times<br/>on a test interface, test class, or test method. Specifically, these annotations will be<br/>found if they are directly present, indirectly present, or meta-present on a given element.|
|---|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

#### [](#writing-tests-conditional-execution-environment-variables)2.8.5. Environment Variable Conditions ####

A container or test may be enabled or disabled based on the value of the `named`environment variable from the underlying operating system via the`[@EnabledIfEnvironmentVariable](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/EnabledIfEnvironmentVariable.html)` and `[@DisabledIfEnvironmentVariable](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/DisabledIfEnvironmentVariable.html)` annotations. The
value supplied via the `matches` attribute will be interpreted as a regular expression.

```
@Test
@EnabledIfEnvironmentVariable(named = "ENV", matches = "staging-server")
void onlyOnStagingServer() {
    // ...
}

@Test
@DisabledIfEnvironmentVariable(named = "ENV", matches = ".*development.*")
void notOnDeveloperWorkstation() {
    // ...
}
```

|   |As of JUnit Jupiter 5.6, `[@EnabledIfEnvironmentVariable](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/EnabledIfEnvironmentVariable.html)` and`[@DisabledIfEnvironmentVariable](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/DisabledIfEnvironmentVariable.html)` are *repeatable annotations*. Consequently, these<br/>annotations may be declared multiple times on a test interface, test class, or test<br/>method. Specifically, these annotations will be found if they are directly present,<br/>indirectly present, or meta-present on a given element.|
|---|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

#### [](#writing-tests-conditional-execution-custom)2.8.6. Custom Conditions ####

As an alternative to implementing an [`ExecutionCondition`](#extensions-conditions), a
container or test may be enabled or disabled based on a *condition method* configured via
the `[@EnabledIf](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/EnabledIf.html)` and `[@DisabledIf](../api/org.junit.jupiter.api/org/junit/jupiter/api/condition/DisabledIf.html)` annotations. A condition method must have a `boolean`return type and may accept either no arguments or a single `ExtensionContext` argument.

The following test class demonstrates how to configure a local method named`customCondition` via `@EnabledIf` and `@DisabledIf`.

```
@Test
@EnabledIf("customCondition")
void enabled() {
    // ...
}

@Test
@DisabledIf("customCondition")
void disabled() {
    // ...
}

boolean customCondition() {
    return true;
}
```

Alternatively, the condition method can be located outside the test class. In this case,
it must be referenced by its *fully qualified name* as demonstrated in the following
example.

```
package example;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.condition.EnabledIf;

class ExternalCustomConditionDemo {

    @Test
    @EnabledIf("example.ExternalCondition#customCondition")
    void enabled() {
        // ...
    }

}

class ExternalCondition {

    static boolean customCondition() {
        return true;
    }

}
```

|   |There are several cases where a condition method would need to be `static`:<br/><br/>* when `@EnabledIf` or `@DisabledIf` is used at class level<br/><br/>* when `@EnabledIf` or `@DisabledIf` is used on a `@ParameterizedTest` or a`@TestTemplate` method<br/><br/>* when the condition method is located in an external class<br/><br/>In any other case, you can use either static methods or instance methods as condition<br/>methods.|
|---|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

|   |It is often the case that you can use an existing static method in a utility class as a<br/>custom condition.<br/><br/>For example, `java.awt.GraphicsEnvironment` provides a `public static boolean isHeadless()`method that can be used to determine if the current environment does not support a<br/>graphical display. Thus, if you have a test that depends on graphical support you can<br/>disable it when such support is unavailable as follows.<br/><br/>```<br/>@DisabledIf(value = "java.awt.GraphicsEnvironment#isHeadless",<br/>    disabledReason = "headless environment")<br/>```|
|---|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

### [](#writing-tests-tagging-and-filtering)2.9. Tagging and Filtering ###

Test classes and methods can be tagged via the `@Tag` annotation. Those tags can later be
used to filter [test discovery and execution](#running-tests). Please refer to the[Tags](#running-tests-tags) section for more information about tag support in the JUnit
Platform.

```
import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

@Tag("fast")
@Tag("model")
class TaggingDemo {

    @Test
    @Tag("taxes")
    void testingTaxCalculation() {
    }

}
```

|   |See [Meta-Annotations and Composed Annotations](#writing-tests-meta-annotations) for examples demonstrating how to create<br/>custom annotations for tags.|
|---|----------------------------------------------------------------------------------------------------------------------------------------------------------|

### [](#writing-tests-test-execution-order)2.10. Test Execution Order ###

By default, test classes and methods will be ordered using an algorithm that is
deterministic but intentionally nonobvious. This ensures that subsequent runs of a test
suite execute test classes and test methods in the same order, thereby allowing for
repeatable builds.

|   |See [Definitions](#writing-tests-definitions) for a definition of *test method* and *test class*.|
|---|-------------------------------------------------------------------------------------------------|

#### [](#writing-tests-test-execution-order-methods)2.10.1. Method Order ####

Although true *unit tests* typically should not rely on the order in which they are
executed, there are times when it is necessary to enforce a specific test method execution
order — for example, when writing *integration tests* or *functional tests* where the
sequence of the tests is important, especially in conjunction with`@TestInstance(Lifecycle.PER_CLASS)`.

To control the order in which test methods are executed, annotate your test class or test
interface with `[@TestMethodOrder](../api/org.junit.jupiter.api/org/junit/jupiter/api/TestMethodOrder.html)` and specify the desired `[MethodOrderer](../api/org.junit.jupiter.api/org/junit/jupiter/api/MethodOrderer.html)`implementation. You can implement your own custom `MethodOrderer` or use one of the
following built-in `MethodOrderer` implementations.

* `[MethodOrderer.DisplayName](../api/org.junit.jupiter.api/org/junit/jupiter/api/MethodOrderer.DisplayName.html)`: sorts test methods *alphanumerically* based on their
  display names (see [display name
  generation precedence rules](#writing-tests-display-name-generator-precedence-rules))

* `[MethodOrderer.MethodName](../api/org.junit.jupiter.api/org/junit/jupiter/api/MethodOrderer.MethodName.html)`: sorts test methods *alphanumerically* based on their names
  and formal parameter lists

* `[MethodOrderer.OrderAnnotation](../api/org.junit.jupiter.api/org/junit/jupiter/api/MethodOrderer.OrderAnnotation.html)`: sorts test methods *numerically* based on values
  specified via the `[@Order](../api/org.junit.jupiter.api/org/junit/jupiter/api/Order.html)` annotation

* `[MethodOrderer.Random](../api/org.junit.jupiter.api/org/junit/jupiter/api/MethodOrderer.Random.html)`: orders test methods *pseudo-randomly* and supports
  configuration of a custom *seed*

* `[MethodOrderer.Alphanumeric](../api/org.junit.jupiter.api/org/junit/jupiter/api/MethodOrderer.Alphanumeric.html)`: sorts test methods *alphanumerically* based on their
  names and formal parameter lists; **deprecated in favor of `[MethodOrderer.MethodName](../api/org.junit.jupiter.api/org/junit/jupiter/api/MethodOrderer.MethodName.html)`,
  to be removed in 6.0**

|   |See also: [Wrapping Behavior of Callbacks](#extensions-execution-order-wrapping-behavior)|
|---|-----------------------------------------------------------------------------------------|

The following example demonstrates how to guarantee that test methods are executed in the
order specified via the `@Order` annotation.

```
import org.junit.jupiter.api.MethodOrderer.OrderAnnotation;
import org.junit.jupiter.api.Order;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestMethodOrder;

@TestMethodOrder(OrderAnnotation.class)
class OrderedTestsDemo {

    @Test
    @Order(1)
    void nullValues() {
        // perform assertions against null values
    }

    @Test
    @Order(2)
    void emptyValues() {
        // perform assertions against empty values
    }

    @Test
    @Order(3)
    void validValues() {
        // perform assertions against valid values
    }

}
```

##### [](#writing-tests-test-execution-order-methods-default)Setting the Default Method Orderer #####

You can use the `junit.jupiter.testmethod.order.default` [configuration parameter](#running-tests-config-params) to specify the fully qualified class name of the`[MethodOrderer](../api/org.junit.jupiter.api/org/junit/jupiter/api/MethodOrderer.html)` you would like to use by default. Just like for the orderer configured
via the `[@TestMethodOrder](../api/org.junit.jupiter.api/org/junit/jupiter/api/TestMethodOrder.html)` annotation, the supplied class has to implement the`MethodOrderer` interface. The default orderer will be used for all tests unless the`@TestMethodOrder` annotation is present on an enclosing test class or test interface.

For example, to use the `[MethodOrderer.OrderAnnotation](../api/org.junit.jupiter.api/org/junit/jupiter/api/MethodOrderer.OrderAnnotation.html)` method orderer by default, you
should set the configuration parameter to the corresponding fully qualified class name
(e.g., in `src/test/resources/junit-platform.properties`):

```
junit.jupiter.testmethod.order.default = \
    org.junit.jupiter.api.MethodOrderer$OrderAnnotation
```

Similarly, you can specify the fully qualified name of any custom class that implements`MethodOrderer`.

#### [](#writing-tests-test-execution-order-classes)2.10.2. Class Order ####

Although test classes typically should not rely on the order in which they are executed,
there are times when it is desirable to enforce a specific test class execution order. You
may wish to execute test classes in a random order to ensure there are no accidental
dependencies between test classes, or you may wish to order test classes to optimize build
time as outlined in the following scenarios.

* Run previously failing tests and faster tests first: "fail fast" mode

* With parallel execution enabled, schedule longer tests first: "shortest test plan
  execution duration" mode

* Various other use cases

To configure test class execution order *globally* for the entire test suite, use the`junit.jupiter.testclass.order.default` [configuration
parameter](#running-tests-config-params) to specify the fully qualified class name of the `[ClassOrderer](../api/org.junit.jupiter.api/org/junit/jupiter/api/ClassOrderer.html)` you would
like to use. The supplied class must implement the `ClassOrderer` interface.

You can implement your own custom `ClassOrderer` or use one of the following built-in`ClassOrderer` implementations.

* `[ClassOrderer.ClassName](../api/org.junit.jupiter.api/org/junit/jupiter/api/ClassOrderer.ClassName.html)`: sorts test classes *alphanumerically* based on their fully
  qualified class names

* `[ClassOrderer.DisplayName](../api/org.junit.jupiter.api/org/junit/jupiter/api/ClassOrderer.DisplayName.html)`: sorts test classes *alphanumerically* based on their
  display names (see [display name
  generation precedence rules](#writing-tests-display-name-generator-precedence-rules))

* `[ClassOrderer.OrderAnnotation](../api/org.junit.jupiter.api/org/junit/jupiter/api/ClassOrderer.OrderAnnotation.html)`: sorts test classes *numerically* based on values
  specified via the `[@Order](../api/org.junit.jupiter.api/org/junit/jupiter/api/Order.html)` annotation

* `[ClassOrderer.Random](../api/org.junit.jupiter.api/org/junit/jupiter/api/ClassOrderer.Random.html)`: orders test classes *pseudo-randomly* and supports
  configuration of a custom *seed*

For example, for the `@Order` annotation to be honored on *test classes*, you should
configure the `[ClassOrderer.OrderAnnotation](../api/org.junit.jupiter.api/org/junit/jupiter/api/ClassOrderer.OrderAnnotation.html)` class orderer using the configuration
parameter with the corresponding fully qualified class name (e.g., in`src/test/resources/junit-platform.properties`):

```
junit.jupiter.testclass.order.default = \
    org.junit.jupiter.api.ClassOrderer$OrderAnnotation
```

The configured `ClassOrderer` will be applied to all top-level test classes (including`static` nested test classes) and `@Nested` test classes.

|   |Top-level test classes will be ordered relative to each other; whereas, `@Nested`test classes will be ordered relative to other `@Nested` test classes sharing the same*enclosing class*.|
|---|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

To configure test class execution order *locally* for `@Nested` test classes, declare the`[@TestClassOrder](../api/org.junit.jupiter.api/org/junit/jupiter/api/TestClassOrder.html)` annotation on the enclosing class for the `@Nested` test classes you
want to order, and supply a class reference to the `ClassOrderer` implementation you would
like to use directly in the `@TestClassOrder` annotation. The configured `ClassOrderer`will be applied recursively to `@Nested` test classes and their `@Nested` test classes.
Note that a local `@TestClassOrder` declaration always overrides an inherited`@TestClassOrder` declaration or a `ClassOrderer` configured globally via the`junit.jupiter.testclass.order.default` configuration parameter.

The following example demonstrates how to guarantee that `@Nested` test classes are
executed in the order specified via the `@Order` annotation.

```
import org.junit.jupiter.api.ClassOrderer;
import org.junit.jupiter.api.Nested;
import org.junit.jupiter.api.Order;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestClassOrder;

@TestClassOrder(ClassOrderer.OrderAnnotation.class)
class OrderedNestedTestClassesDemo {

    @Nested
    @Order(1)
    class PrimaryTests {

        @Test
        void test1() {
        }
    }

    @Nested
    @Order(2)
    class SecondaryTests {

        @Test
        void test2() {
        }
    }
}
```

### [](#writing-tests-test-instance-lifecycle)2.11. Test Instance Lifecycle ###

In order to allow individual test methods to be executed in isolation and to avoid
unexpected side effects due to mutable test instance state, JUnit creates a new instance
of each test class before executing each *test method* (see[Definitions](#writing-tests-definitions)). This "per-method" test instance lifecycle is the default
behavior in JUnit Jupiter and is analogous to all previous versions of JUnit.

|   |Please note that the test class will still be instantiated if a given *test method*is *disabled* via a [condition](#writing-tests-conditional-execution) (e.g., `@Disabled`,`@DisabledOnOs`, etc.) even when the "per-method" test instance lifecycle mode is active.|
|---|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

If you would prefer that JUnit Jupiter execute all test methods on the same test
instance, annotate your test class with `@TestInstance(Lifecycle.PER_CLASS)`. When using
this mode, a new test instance will be created once per test class. Thus, if your test
methods rely on state stored in instance variables, you may need to reset that state in`@BeforeEach` or `@AfterEach` methods.

The "per-class" mode has some additional benefits over the default "per-method" mode.
Specifically, with the "per-class" mode it becomes possible to declare `@BeforeAll` and`@AfterAll` on non-static methods as well as on interface `default` methods. The
"per-class" mode therefore also makes it possible to use `@BeforeAll` and `@AfterAll`methods in `@Nested` test classes.

|   |Beginning with Java 16, `@BeforeAll` and `@AfterAll` methods can be declared as`static` in `@Nested` test classes.|
|---|------------------------------------------------------------------------------------------------------------------|

If you are authoring tests using the Kotlin programming language, you may also find it
easier to implement non-static `@BeforeAll` and `@AfterAll` lifecycle methods as well as`@MethodSource` factory methods by switching to the "per-class" test instance lifecycle
mode.

#### [](#writing-tests-test-instance-lifecycle-changing-default)2.11.1. Changing the Default Test Instance Lifecycle ####

If a test class or test interface is not annotated with `@TestInstance`, JUnit Jupiter
will use a *default* lifecycle mode. The standard *default* mode is `PER_METHOD`;
however, it is possible to change the *default* for the execution of an entire test plan.
To change the default test instance lifecycle mode, set the`junit.jupiter.testinstance.lifecycle.default` *configuration parameter* to the name of
an enum constant defined in `TestInstance.Lifecycle`, ignoring case. This can be supplied
as a JVM system property, as a *configuration parameter* in the`LauncherDiscoveryRequest` that is passed to the `Launcher`, or via the JUnit Platform
configuration file (see [Configuration Parameters](#running-tests-config-params) for details).

For example, to set the default test instance lifecycle mode to `Lifecycle.PER_CLASS`,
you can start your JVM with the following system property.

`-Djunit.jupiter.testinstance.lifecycle.default=per_class`

Note, however, that setting the default test instance lifecycle mode via the JUnit
Platform configuration file is a more robust solution since the configuration file can be
checked into a version control system along with your project and can therefore be used
within IDEs and your build software.

To set the default test instance lifecycle mode to `Lifecycle.PER_CLASS` via the JUnit
Platform configuration file, create a file named `junit-platform.properties` in the root
of the class path (e.g., `src/test/resources`) with the following content.

`junit.jupiter.testinstance.lifecycle.default = per_class`

|   |Changing the *default* test instance lifecycle mode can lead to unpredictable<br/>results and fragile builds if not applied consistently. For example, if the build<br/>configures "per-class" semantics as the default but tests in the IDE are executed using<br/>"per-method" semantics, that can make it difficult to debug errors that occur on the<br/>build server. It is therefore recommended to change the default in the JUnit Platform<br/>configuration file instead of via a JVM system property.|
|---|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

### [](#writing-tests-nested)2.12. Nested Tests ###

`@Nested` tests give the test writer more capabilities to express the relationship among
several groups of tests. Such nested tests make use of Java’s nested classes and
facilitate hierarchical thinking about the test structure. Here’s an elaborate example,
both as source code and as a screenshot of the execution within an IDE.

Nested test suite for testing a stack

```
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.EmptyStackException;
import java.util.Stack;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Nested;
import org.junit.jupiter.api.Test;

@DisplayName("A stack")
class TestingAStackDemo {

    Stack<Object> stack;

    @Test
    @DisplayName("is instantiated with new Stack()")
    void isInstantiatedWithNew() {
        new Stack<>();
    }

    @Nested
    @DisplayName("when new")
    class WhenNew {

        @BeforeEach
        void createNewStack() {
            stack = new Stack<>();
        }

        @Test
        @DisplayName("is empty")
        void isEmpty() {
            assertTrue(stack.isEmpty());
        }

        @Test
        @DisplayName("throws EmptyStackException when popped")
        void throwsExceptionWhenPopped() {
            assertThrows(EmptyStackException.class, stack::pop);
        }

        @Test
        @DisplayName("throws EmptyStackException when peeked")
        void throwsExceptionWhenPeeked() {
            assertThrows(EmptyStackException.class, stack::peek);
        }

        @Nested
        @DisplayName("after pushing an element")
        class AfterPushing {

            String anElement = "an element";

            @BeforeEach
            void pushAnElement() {
                stack.push(anElement);
            }

            @Test
            @DisplayName("it is no longer empty")
            void isNotEmpty() {
                assertFalse(stack.isEmpty());
            }

            @Test
            @DisplayName("returns the element when popped and is empty")
            void returnElementWhenPopped() {
                assertEquals(anElement, stack.pop());
                assertTrue(stack.isEmpty());
            }

            @Test
            @DisplayName("returns the element when peeked but remains not empty")
            void returnElementWhenPeeked() {
                assertEquals(anElement, stack.peek());
                assertFalse(stack.isEmpty());
            }
        }
    }
}
```

When executing this example in an IDE, the test execution tree in the GUI will look
similar to the following image.

![writing tests nested test ide](images/writing-tests_nested_test_ide.png)

Executing a nested test in an IDE

In this example, preconditions from outer tests are used in inner tests by defining
hierarchical lifecycle methods for the setup code. For example, `createNewStack()` is a`@BeforeEach` lifecycle method that is used in the test class in which it is defined and
in all levels in the nesting tree below the class in which it is defined.

The fact that setup code from outer tests is run before inner tests are executed gives you
the ability to run all tests independently. You can even run inner tests alone without
running the outer tests, because the setup code from the outer tests is always executed.

|   |*Only non-static nested classes* (i.e. *inner classes*) can serve as `@Nested` test<br/>classes. Nesting can be arbitrarily deep, and those inner classes are subject to full<br/>lifecycle support with one exception: `@BeforeAll` and `@AfterAll` methods do not work *by<br/>default*. The reason is that Java does not allow `static` members in inner classes prior<br/>to Java 16. However, this restriction can be circumvented by annotating a `@Nested` test<br/>class with `@TestInstance(Lifecycle.PER_CLASS)` (see[Test Instance Lifecycle](#writing-tests-test-instance-lifecycle)). If you are using Java 16 or higher,`@BeforeAll` and `@AfterAll` methods can be declared as `static` in `@Nested` test<br/>classes, and this restriction no longer applies.|
|---|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

### [](#writing-tests-dependency-injection)2.13. Dependency Injection for Constructors and Methods ###

In all prior JUnit versions, test constructors or methods were not allowed to have
parameters (at least not with the standard `Runner` implementations). As one of the major
changes in JUnit Jupiter, both test constructors and methods are now permitted to have
parameters. This allows for greater flexibility and enables *Dependency Injection* for
constructors and methods.

`[ParameterResolver](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/ParameterResolver.html)` defines the API for test extensions that wish to *dynamically*resolve parameters at runtime. If a *test class* constructor, a *test method*, or a*lifecycle method* (see [Definitions](#writing-tests-definitions)) accepts a parameter, the parameter
must be resolved at runtime by a registered `ParameterResolver`.

There are currently three built-in resolvers that are registered automatically.

* `[TestInfoParameterResolver](https://github.com/junit-team/junit5/tree/r5.10.2/junit-jupiter-engine/src/main/java/org/junit/jupiter/engine/extension/TestInfoParameterResolver.java)`: if a constructor or method parameter is of type`[TestInfo](../api/org.junit.jupiter.api/org/junit/jupiter/api/TestInfo.html)`, the `TestInfoParameterResolver` will supply an instance of `TestInfo`corresponding to the current container or test as the value for the parameter. The`TestInfo` can then be used to retrieve information about the current container or test
  such as the display name, the test class, the test method, and associated tags. The
  display name is either a technical name, such as the name of the test class or test
  method, or a custom name configured via `@DisplayName`.

  `[TestInfo](../api/org.junit.jupiter.api/org/junit/jupiter/api/TestInfo.html)` acts as a drop-in replacement for the `TestName` rule from JUnit 4. The
  following demonstrates how to have `TestInfo` injected into a test constructor,`@BeforeEach` method, and `@Test` method.

```
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestInfo;

@DisplayName("TestInfo Demo")
class TestInfoDemo {

    TestInfoDemo(TestInfo testInfo) {
        assertEquals("TestInfo Demo", testInfo.getDisplayName());
    }

    @BeforeEach
    void init(TestInfo testInfo) {
        String displayName = testInfo.getDisplayName();
        assertTrue(displayName.equals("TEST 1") || displayName.equals("test2()"));
    }

    @Test
    @DisplayName("TEST 1")
    @Tag("my-tag")
    void test1(TestInfo testInfo) {
        assertEquals("TEST 1", testInfo.getDisplayName());
        assertTrue(testInfo.getTags().contains("my-tag"));
    }

    @Test
    void test2() {
    }

}
```

* `[RepetitionExtension](https://github.com/junit-team/junit5/tree/r5.10.2/junit-jupiter-engine/src/main/java/org/junit/jupiter/engine/extension/RepetitionExtension.java)`: if a method parameter in a `@RepeatedTest`, `@BeforeEach`, or`@AfterEach` method is of type `[RepetitionInfo](../api/org.junit.jupiter.api/org/junit/jupiter/api/RepetitionInfo.html)`, the `RepetitionExtension` will supply
  an instance of `RepetitionInfo`. `RepetitionInfo` can then be used to retrieve
  information about the current repetition, the total number of repetitions, the number of
  repetitions that have failed, and the failure threshold for the corresponding`@RepeatedTest`. Note, however, that `RepetitionExtension` is not registered outside the
  context of a `@RepeatedTest`. See [Repeated Test Examples](#writing-tests-repeated-tests-examples).

* `[TestReporterParameterResolver](https://github.com/junit-team/junit5/tree/r5.10.2/junit-jupiter-engine/src/main/java/org/junit/jupiter/engine/extension/TestReporterParameterResolver.java)`: if a constructor or method parameter is of type`[TestReporter](../api/org.junit.jupiter.api/org/junit/jupiter/api/TestReporter.html)`, the `TestReporterParameterResolver` will supply an instance of`TestReporter`. The `TestReporter` can be used to publish additional data about the
  current test run. The data can be consumed via the `reportingEntryPublished()` method in
  a `[TestExecutionListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/TestExecutionListener.html)`, allowing it to be viewed in IDEs or included in reports.

  In JUnit Jupiter you should use `TestReporter` where you used to print information to`stdout` or `stderr` in JUnit 4. Using `@RunWith(JUnitPlatform.class)` will output all
  reported entries to `stdout`. In addition, some IDEs print report entries to `stdout` or
  display them in the user interface for test results.

```
class TestReporterDemo {

    @Test
    void reportSingleValue(TestReporter testReporter) {
        testReporter.publishEntry("a status message");
    }

    @Test
    void reportKeyValuePair(TestReporter testReporter) {
        testReporter.publishEntry("a key", "a value");
    }

    @Test
    void reportMultipleKeyValuePairs(TestReporter testReporter) {
        Map<String, String> values = new HashMap<>();
        values.put("user name", "dk38");
        values.put("award year", "1974");

        testReporter.publishEntry(values);
    }

}
```

|   |Other parameter resolvers must be explicitly enabled by registering appropriate[extensions](#extensions) via `@ExtendWith`.|
|---|---------------------------------------------------------------------------------------------------------------------------|

Check out the `[RandomParametersExtension](https://github.com/junit-team/junit5-samples/tree/r5.10.2/junit5-jupiter-extensions/src/main/java/com/example/random/RandomParametersExtension.java)` for an example of a custom`[ParameterResolver](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/ParameterResolver.html)`. While not intended to be production-ready, it demonstrates the
simplicity and expressiveness of both the extension model and the parameter resolution
process. `MyRandomParametersTest` demonstrates how to inject random values into `@Test`methods.

```
@ExtendWith(RandomParametersExtension.class)
class MyRandomParametersTest {

    @Test
    void injectsInteger(@Random int i, @Random int j) {
        assertNotEquals(i, j);
    }

    @Test
    void injectsDouble(@Random double d) {
        assertEquals(0.0, d, 1.0);
    }

}
```

For real-world use cases, check out the source code for the `[MockitoExtension](https://github.com/mockito/mockito/blob/release/2.x/subprojects/junit-jupiter/src/main/java/org/mockito/junit/jupiter/MockitoExtension.java)` and the`[SpringExtension](https://github.com/spring-projects/spring-framework/tree/HEAD/spring-test/src/main/java/org/springframework/test/context/junit/jupiter/SpringExtension.java)`.

When the type of the parameter to inject is the only condition for your`[ParameterResolver](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/ParameterResolver.html)`, you can use the generic `[TypeBasedParameterResolver](https://github.com/junit-team/junit5/tree/r5.10.2/junit-jupiter-api/src/main/java/org/junit/jupiter/api/extension/support/TypeBasedParameterResolver.java)` base class.
The `supportsParameters` method is implemented behind the scenes and supports
parameterized types.

### [](#writing-tests-test-interfaces-and-default-methods)2.14. Test Interfaces and Default Methods ###

JUnit Jupiter allows `@Test`, `@RepeatedTest`, `@ParameterizedTest`, `@TestFactory`,`@TestTemplate`, `@BeforeEach`, and `@AfterEach` to be declared on interface `default`methods. `@BeforeAll` and `@AfterAll` can either be declared on `static` methods in a
test interface or on interface `default` methods *if* the test interface or test class is
annotated with `@TestInstance(Lifecycle.PER_CLASS)` (see[Test Instance Lifecycle](#writing-tests-test-instance-lifecycle)). Here are some examples.

```
@TestInstance(Lifecycle.PER_CLASS)
interface TestLifecycleLogger {

    static final Logger logger = Logger.getLogger(TestLifecycleLogger.class.getName());

    @BeforeAll
    default void beforeAllTests() {
        logger.info("Before all tests");
    }

    @AfterAll
    default void afterAllTests() {
        logger.info("After all tests");
    }

    @BeforeEach
    default void beforeEachTest(TestInfo testInfo) {
        logger.info(() -> String.format("About to execute [%s]",
            testInfo.getDisplayName()));
    }

    @AfterEach
    default void afterEachTest(TestInfo testInfo) {
        logger.info(() -> String.format("Finished executing [%s]",
            testInfo.getDisplayName()));
    }

}
```

```
interface TestInterfaceDynamicTestsDemo {

    @TestFactory
    default Stream<DynamicTest> dynamicTestsForPalindromes() {
        return Stream.of("racecar", "radar", "mom", "dad")
            .map(text -> dynamicTest(text, () -> assertTrue(isPalindrome(text))));
    }

}
```

`@ExtendWith` and `@Tag` can be declared on a test interface so that classes that
implement the interface automatically inherit its tags and extensions. See[Before and After Test Execution Callbacks](#extensions-lifecycle-callbacks-before-after-execution) for the source code of the[TimingExtension](#extensions-lifecycle-callbacks-timing-extension).

```
@Tag("timed")
@ExtendWith(TimingExtension.class)
interface TimeExecutionLogger {
}
```

In your test class you can then implement these test interfaces to have them applied.

```
class TestInterfaceDemo implements TestLifecycleLogger,
        TimeExecutionLogger, TestInterfaceDynamicTestsDemo {

    @Test
    void isEqualValue() {
        assertEquals(1, "a".length(), "is always equal");
    }

}
```

Running the `TestInterfaceDemo` results in output similar to the following:

```
INFO  example.TestLifecycleLogger - Before all tests
INFO  example.TestLifecycleLogger - About to execute [dynamicTestsForPalindromes()]
INFO  example.TimingExtension - Method [dynamicTestsForPalindromes] took 19 ms.
INFO  example.TestLifecycleLogger - Finished executing [dynamicTestsForPalindromes()]
INFO  example.TestLifecycleLogger - About to execute [isEqualValue()]
INFO  example.TimingExtension - Method [isEqualValue] took 1 ms.
INFO  example.TestLifecycleLogger - Finished executing [isEqualValue()]
INFO  example.TestLifecycleLogger - After all tests
```

Another possible application of this feature is to write tests for interface contracts.
For example, you can write tests for how implementations of `Object.equals` or`Comparable.compareTo` should behave as follows.

```
public interface Testable<T> {

    T createValue();

}
```

```
public interface EqualsContract<T> extends Testable<T> {

    T createNotEqualValue();

    @Test
    default void valueEqualsItself() {
        T value = createValue();
        assertEquals(value, value);
    }

    @Test
    default void valueDoesNotEqualNull() {
        T value = createValue();
        assertFalse(value.equals(null));
    }

    @Test
    default void valueDoesNotEqualDifferentValue() {
        T value = createValue();
        T differentValue = createNotEqualValue();
        assertNotEquals(value, differentValue);
        assertNotEquals(differentValue, value);
    }

}
```

```
public interface ComparableContract<T extends Comparable<T>> extends Testable<T> {

    T createSmallerValue();

    @Test
    default void returnsZeroWhenComparedToItself() {
        T value = createValue();
        assertEquals(0, value.compareTo(value));
    }

    @Test
    default void returnsPositiveNumberWhenComparedToSmallerValue() {
        T value = createValue();
        T smallerValue = createSmallerValue();
        assertTrue(value.compareTo(smallerValue) > 0);
    }

    @Test
    default void returnsNegativeNumberWhenComparedToLargerValue() {
        T value = createValue();
        T smallerValue = createSmallerValue();
        assertTrue(smallerValue.compareTo(value) < 0);
    }

}
```

In your test class you can then implement both contract interfaces thereby inheriting the
corresponding tests. Of course you’ll have to implement the abstract methods.

```
class StringTests implements ComparableContract<String>, EqualsContract<String> {

    @Override
    public String createValue() {
        return "banana";
    }

    @Override
    public String createSmallerValue() {
        return "apple"; // 'a' < 'b' in "banana"
    }

    @Override
    public String createNotEqualValue() {
        return "cherry";
    }

}
```

|   |The above tests are merely meant as examples and therefore not complete.|
|---|------------------------------------------------------------------------|

### [](#writing-tests-repeated-tests)2.15. Repeated Tests ###

JUnit Jupiter provides the ability to repeat a test a specified number of times by
annotating a method with `@RepeatedTest` and specifying the total number of repetitions
desired. Each invocation of a repeated test behaves like the execution of a regular`@Test` method with full support for the same lifecycle callbacks and extensions.

The following example demonstrates how to declare a test named `repeatedTest()` that
will be automatically repeated 10 times.

```
@RepeatedTest(10)
void repeatedTest() {
    // ...
}
```

Since JUnit Jupiter 5.10, `@RepeatedTest` can be configured with a failure threshold which
signifies the number of failures after which remaining repetitions will be automatically
skipped. Set the `failureThreshold` attribute to a positive number less than the total
number of repetitions in order to skip the invocations of remaining repetitions after the
specified number of failures has been encountered.

For example, if you are using `@RepeatedTest` to repeatedly invoke a test that you suspect
to be *flaky*, a single failure is sufficient to demonstrate that the test is flaky, and
there is no need to invoke the remaining repetitions. To support that specific use case,
set `failureThreshold = 1`. You can alternatively set the threshold to a number greater
than 1 depending on your use case.

By default, the `failureThreshold` attribute is set to `Integer.MAX_VALUE`, signaling that
no failure threshold will be applied, which effectively means that the specified number of
repetitions will be invoked regardless of whether any repetitions fail.

|   |If the repetitions of a `@RepeatedTest` method are executed in parallel, no<br/>guarantees can be made regarding the failure threshold. It is therefore recommended that a`@RepeatedTest` method be annotated with `@Execution(SAME_THREAD)` when parallel execution<br/>is configured. See [Parallel Execution](#writing-tests-parallel-execution) for further details.|
|---|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

In addition to specifying the number of repetitions and failure threshold, a custom
display name can be configured for each repetition via the `name` attribute of the`@RepeatedTest` annotation. Furthermore, the display name can be a pattern composed of a
combination of static text and dynamic placeholders. The following placeholders are
currently supported.

* `{displayName}`: display name of the `@RepeatedTest` method

* `{currentRepetition}`: the current repetition count

* `{totalRepetitions}`: the total number of repetitions

The default display name for a given repetition is generated based on the following
pattern: `"repetition {currentRepetition} of {totalRepetitions}"`. Thus, the display
names for individual repetitions of the previous `repeatedTest()` example would be:`repetition 1 of 10`, `repetition 2 of 10`, etc. If you would like the display name of
the `@RepeatedTest` method included in the name of each repetition, you can define your
own custom pattern or use the predefined `RepeatedTest.LONG_DISPLAY_NAME` pattern. The
latter is equal to `"{displayName} :: repetition {currentRepetition} of
{totalRepetitions}"` which results in display names for individual repetitions like`repeatedTest() :: repetition 1 of 10`, `repeatedTest() :: repetition 2 of 10`, etc.

In order to retrieve information about the current repetition, the total number of
repetitions, the number of repetitions that have failed, and the failure threshold, a
developer can choose to have an instance of `[RepetitionInfo](../api/org.junit.jupiter.api/org/junit/jupiter/api/RepetitionInfo.html)` injected into a`@RepeatedTest`, `@BeforeEach`, or `@AfterEach` method.

#### [](#writing-tests-repeated-tests-examples)2.15.1. Repeated Test Examples ####

The `RepeatedTestsDemo` class at the end of this section demonstrates several examples of
repeated tests.

The `repeatedTest()` method is identical to the example from the previous section; whereas,`repeatedTestWithRepetitionInfo()` demonstrates how to have an instance of`RepetitionInfo` injected into a test to access the total number of repetitions for the
current repeated test.

`repeatedTestWithFailureThreshold()` demonstrates how to set a failure threshold and
simulates an unexpected failure for every second repetition. The resulting behavior can be
viewed in the `ConsoleLauncher` output at the end of this section.

The next two methods demonstrate how to include a custom `@DisplayName` for the`@RepeatedTest` method in the display name of each repetition. `customDisplayName()`combines a custom display name with a custom pattern and then uses `TestInfo` to verify
the format of the generated display name. `Repeat!` is the `{displayName}` which comes
from the `@DisplayName` declaration, and `1/1` comes from`{currentRepetition}/{totalRepetitions}`. In contrast,`customDisplayNameWithLongPattern()` uses the aforementioned predefined`RepeatedTest.LONG_DISPLAY_NAME` pattern.

`repeatedTestInGerman()` demonstrates the ability to translate display names of repeated
tests into foreign languages — in this case German, resulting in names for individual
repetitions such as: `Wiederholung 1 von 5`, `Wiederholung 2 von 5`, etc.

Since the `beforeEach()` method is annotated with `@BeforeEach` it will get executed
before each repetition of each repeated test. By having the `TestInfo` and`RepetitionInfo` injected into the method, we see that it’s possible to obtain
information about the currently executing repeated test. Executing `RepeatedTestsDemo`with the `INFO` log level enabled results in the following output.

```
INFO: About to execute repetition 1 of 10 for repeatedTest
INFO: About to execute repetition 2 of 10 for repeatedTest
INFO: About to execute repetition 3 of 10 for repeatedTest
INFO: About to execute repetition 4 of 10 for repeatedTest
INFO: About to execute repetition 5 of 10 for repeatedTest
INFO: About to execute repetition 6 of 10 for repeatedTest
INFO: About to execute repetition 7 of 10 for repeatedTest
INFO: About to execute repetition 8 of 10 for repeatedTest
INFO: About to execute repetition 9 of 10 for repeatedTest
INFO: About to execute repetition 10 of 10 for repeatedTest
INFO: About to execute repetition 1 of 5 for repeatedTestWithRepetitionInfo
INFO: About to execute repetition 2 of 5 for repeatedTestWithRepetitionInfo
INFO: About to execute repetition 3 of 5 for repeatedTestWithRepetitionInfo
INFO: About to execute repetition 4 of 5 for repeatedTestWithRepetitionInfo
INFO: About to execute repetition 5 of 5 for repeatedTestWithRepetitionInfo
INFO: About to execute repetition 1 of 8 for repeatedTestWithFailureThreshold
INFO: About to execute repetition 2 of 8 for repeatedTestWithFailureThreshold
INFO: About to execute repetition 3 of 8 for repeatedTestWithFailureThreshold
INFO: About to execute repetition 4 of 8 for repeatedTestWithFailureThreshold
INFO: About to execute repetition 1 of 1 for customDisplayName
INFO: About to execute repetition 1 of 1 for customDisplayNameWithLongPattern
INFO: About to execute repetition 1 of 5 for repeatedTestInGerman
INFO: About to execute repetition 2 of 5 for repeatedTestInGerman
INFO: About to execute repetition 3 of 5 for repeatedTestInGerman
INFO: About to execute repetition 4 of 5 for repeatedTestInGerman
INFO: About to execute repetition 5 of 5 for repeatedTestInGerman
```

```
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.fail;

import java.util.logging.Logger;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.RepeatedTest;
import org.junit.jupiter.api.RepetitionInfo;
import org.junit.jupiter.api.TestInfo;

class RepeatedTestsDemo {

    private Logger logger = // ...

    @BeforeEach
    void beforeEach(TestInfo testInfo, RepetitionInfo repetitionInfo) {
        int currentRepetition = repetitionInfo.getCurrentRepetition();
        int totalRepetitions = repetitionInfo.getTotalRepetitions();
        String methodName = testInfo.getTestMethod().get().getName();
        logger.info(String.format("About to execute repetition %d of %d for %s", //
            currentRepetition, totalRepetitions, methodName));
    }

    @RepeatedTest(10)
    void repeatedTest() {
        // ...
    }

    @RepeatedTest(5)
    void repeatedTestWithRepetitionInfo(RepetitionInfo repetitionInfo) {
        assertEquals(5, repetitionInfo.getTotalRepetitions());
    }

    @RepeatedTest(value = 8, failureThreshold = 2)
    void repeatedTestWithFailureThreshold(RepetitionInfo repetitionInfo) {
        // Simulate unexpected failure every second repetition
        if (repetitionInfo.getCurrentRepetition() % 2 == 0) {
            fail("Boom!");
        }
    }

    @RepeatedTest(value = 1, name = "{displayName} {currentRepetition}/{totalRepetitions}")
    @DisplayName("Repeat!")
    void customDisplayName(TestInfo testInfo) {
        assertEquals("Repeat! 1/1", testInfo.getDisplayName());
    }

    @RepeatedTest(value = 1, name = RepeatedTest.LONG_DISPLAY_NAME)
    @DisplayName("Details...")
    void customDisplayNameWithLongPattern(TestInfo testInfo) {
        assertEquals("Details... :: repetition 1 of 1", testInfo.getDisplayName());
    }

    @RepeatedTest(value = 5, name = "Wiederholung {currentRepetition} von {totalRepetitions}")
    void repeatedTestInGerman() {
        // ...
    }

}
```

When using the `ConsoleLauncher` with the unicode theme enabled, execution of`RepeatedTestsDemo` results in the following output to the console.

```
├─ RepeatedTestsDemo ✔
│  ├─ repeatedTest() ✔
│  │  ├─ repetition 1 of 10 ✔
│  │  ├─ repetition 2 of 10 ✔
│  │  ├─ repetition 3 of 10 ✔
│  │  ├─ repetition 4 of 10 ✔
│  │  ├─ repetition 5 of 10 ✔
│  │  ├─ repetition 6 of 10 ✔
│  │  ├─ repetition 7 of 10 ✔
│  │  ├─ repetition 8 of 10 ✔
│  │  ├─ repetition 9 of 10 ✔
│  │  └─ repetition 10 of 10 ✔
│  ├─ repeatedTestWithRepetitionInfo(RepetitionInfo) ✔
│  │  ├─ repetition 1 of 5 ✔
│  │  ├─ repetition 2 of 5 ✔
│  │  ├─ repetition 3 of 5 ✔
│  │  ├─ repetition 4 of 5 ✔
│  │  └─ repetition 5 of 5 ✔
│  ├─ repeatedTestWithFailureThreshold(RepetitionInfo) ✔
│  │  ├─ repetition 1 of 8 ✔
│  │  ├─ repetition 2 of 8 ✘ Boom!
│  │  ├─ repetition 3 of 8 ✔
│  │  ├─ repetition 4 of 8 ✘ Boom!
│  │  ├─ repetition 5 of 8 ↷ Failure threshold [2] exceeded
│  │  ├─ repetition 6 of 8 ↷ Failure threshold [2] exceeded
│  │  ├─ repetition 7 of 8 ↷ Failure threshold [2] exceeded
│  │  └─ repetition 8 of 8 ↷ Failure threshold [2] exceeded
│  ├─ Repeat! ✔
│  │  └─ Repeat! 1/1 ✔
│  ├─ Details... ✔
│  │  └─ Details... :: repetition 1 of 1 ✔
│  └─ repeatedTestInGerman() ✔
│     ├─ Wiederholung 1 von 5 ✔
│     ├─ Wiederholung 2 von 5 ✔
│     ├─ Wiederholung 3 von 5 ✔
│     ├─ Wiederholung 4 von 5 ✔
│     └─ Wiederholung 5 von 5 ✔
```

### [](#writing-tests-parameterized-tests)2.16. Parameterized Tests ###

Parameterized tests make it possible to run a test multiple times with different
arguments. They are declared just like regular `@Test` methods but use the`[@ParameterizedTest](../api/org.junit.jupiter.params/org/junit/jupiter/params/ParameterizedTest.html)` annotation instead. In addition, you must declare at least one*source* that will provide the arguments for each invocation and then *consume* the
arguments in the test method.

The following example demonstrates a parameterized test that uses the `@ValueSource`annotation to specify a `String` array as the source of arguments.

```
@ParameterizedTest
@ValueSource(strings = { "racecar", "radar", "able was I ere I saw elba" })
void palindromes(String candidate) {
    assertTrue(StringUtils.isPalindrome(candidate));
}
```

When executing the above parameterized test method, each invocation will be reported
separately. For instance, the `ConsoleLauncher` will print output similar to the
following.

```
palindromes(String) ✔
├─ [1] candidate=racecar ✔
├─ [2] candidate=radar ✔
└─ [3] candidate=able was I ere I saw elba ✔
```

#### [](#writing-tests-parameterized-tests-setup)2.16.1. Required Setup ####

In order to use parameterized tests you need to add a dependency on the`junit-jupiter-params` artifact. Please refer to [Dependency Metadata](#dependency-metadata) for details.

#### [](#writing-tests-parameterized-tests-consuming-arguments)2.16.2. Consuming Arguments ####

Parameterized test methods typically *consume* arguments directly from the configured
source (see [Sources of Arguments](#writing-tests-parameterized-tests-sources)) following a one-to-one
correlation between argument source index and method parameter index (see examples in[@CsvSource](#writing-tests-parameterized-tests-sources-CsvSource)). However, a parameterized test
method may also choose to *aggregate* arguments from the source into a single object
passed to the method (see [Argument Aggregation](#writing-tests-parameterized-tests-argument-aggregation)).
Additional arguments may also be provided by a `ParameterResolver` (e.g., to obtain an
instance of `TestInfo`, `TestReporter`, etc.). Specifically, a parameterized test method
must declare formal parameters according to the following rules.

* Zero or more *indexed arguments* must be declared first.

* Zero or more *aggregators* must be declared next.

* Zero or more arguments supplied by a `ParameterResolver` must be declared last.

In this context, an *indexed argument* is an argument for a given index in the`Arguments` provided by an `ArgumentsProvider` that is passed as an argument to the
parameterized method at the same index in the method’s formal parameter list. An*aggregator* is any parameter of type `ArgumentsAccessor` or any parameter annotated with`@AggregateWith`.

|   |AutoCloseable arguments<br/><br/>Arguments that implement `java.lang.AutoCloseable` (or `java.io.Closeable` which extends`java.lang.AutoCloseable`) will be automatically closed after `@AfterEach` methods and`AfterEachCallback` extensions have been called for the current parameterized test<br/>invocation.<br/><br/>To prevent this from happening, set the `autoCloseArguments` attribute in`@ParameterizedTest` to `false`. Specifically, if an argument that implements`AutoCloseable` is reused for multiple invocations of the same parameterized test method,<br/>you must annotate the method with `@ParameterizedTest(autoCloseArguments = false)` to<br/>ensure that the argument is not closed between invocations.|
|---|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

#### [](#writing-tests-parameterized-tests-sources)2.16.3. Sources of Arguments ####

Out of the box, JUnit Jupiter provides quite a few *source* annotations. Each of the
following subsections provides a brief overview and an example for each of them. Please
refer to the Javadoc in the `[org.junit.jupiter.params.provider](../api/org.junit.jupiter.params/org/junit/jupiter/params/provider/package-summary.html)` package for additional
information.

##### [](#writing-tests-parameterized-tests-sources-ValueSource)@ValueSource #####

`@ValueSource` is one of the simplest possible sources. It lets you specify a single
array of literal values and can only be used for providing a single argument per
parameterized test invocation.

The following types of literal values are supported by `@ValueSource`.

* `short`

* `byte`

* `int`

* `long`

* `float`

* `double`

* `char`

* `boolean`

* `java.lang.String`

* `java.lang.Class`

For example, the following `@ParameterizedTest` method will be invoked three times, with
the values `1`, `2`, and `3` respectively.

```
@ParameterizedTest
@ValueSource(ints = { 1, 2, 3 })
void testWithValueSource(int argument) {
    assertTrue(argument > 0 && argument < 4);
}
```

##### [](#writing-tests-parameterized-tests-sources-null-and-empty)Null and Empty Sources #####

In order to check corner cases and verify proper behavior of our software when it is
supplied *bad input*, it can be useful to have `null` and *empty* values supplied to our
parameterized tests. The following annotations serve as sources of `null` and empty values
for parameterized tests that accept a single argument.

* `[@NullSource](../api/org.junit.jupiter.params/org/junit/jupiter/params/provider/NullSource.html)`: provides a single `null` argument to the annotated `@ParameterizedTest`method.

  * `@NullSource` cannot be used for a parameter that has a primitive type.

* `[@EmptySource](../api/org.junit.jupiter.params/org/junit/jupiter/params/provider/EmptySource.html)`: provides a single *empty* argument to the annotated`@ParameterizedTest` method for parameters of the following types: `java.lang.String`,`java.util.Collection` (and concrete subtypes with a `public` no-arg constructor),`java.util.List`, `java.util.Set`, `java.util.SortedSet`, `java.util.NavigableSet`,`java.util.Map` (and concrete subtypes with a `public` no-arg constructor),`java.util.SortedMap`, `java.util.NavigableMap`, primitive arrays (e.g., `int[]`,`char[][]`, etc.), object arrays (e.g., `String[]`, `Integer[][]`, etc.).

* `[@NullAndEmptySource](../api/org.junit.jupiter.params/org/junit/jupiter/params/provider/NullAndEmptySource.html)`: a *composed annotation* that combines the functionality of`@NullSource` and `@EmptySource`.

If you need to supply multiple varying types of *blank* strings to a parameterized test,
you can achieve that using [@ValueSource](#writing-tests-parameterized-tests-sources-ValueSource) — for example, `@ValueSource(strings = {" ", " ", "\t", "\n"})`.

You can also combine `@NullSource`, `@EmptySource`, and `@ValueSource` to test a wider
range of `null`, *empty*, and *blank* input. The following example demonstrates how to
achieve this for strings.

```
@ParameterizedTest
@NullSource
@EmptySource
@ValueSource(strings = { " ", "   ", "\t", "\n" })
void nullEmptyAndBlankStrings(String text) {
    assertTrue(text == null || text.trim().isEmpty());
}
```

Making use of the composed `@NullAndEmptySource` annotation simplifies the above as
follows.

```
@ParameterizedTest
@NullAndEmptySource
@ValueSource(strings = { " ", "   ", "\t", "\n" })
void nullEmptyAndBlankStrings(String text) {
    assertTrue(text == null || text.trim().isEmpty());
}
```

|   |Both variants of the `nullEmptyAndBlankStrings(String)` parameterized test method<br/>result in six invocations: 1 for `null`, 1 for the empty string, and 4 for the explicit<br/>blank strings supplied via `@ValueSource`.|
|---|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

##### [](#writing-tests-parameterized-tests-sources-EnumSource)@EnumSource #####

`@EnumSource` provides a convenient way to use `Enum` constants.

```
@ParameterizedTest
@EnumSource(ChronoUnit.class)
void testWithEnumSource(TemporalUnit unit) {
    assertNotNull(unit);
}
```

The annotation’s `value` attribute is optional. When omitted, the declared type of the
first method parameter is used. The test will fail if it does not reference an enum type.
Thus, the `value` attribute is required in the above example because the method parameter
is declared as `TemporalUnit`, i.e. the interface implemented by `ChronoUnit`, which isn’t
an enum type. Changing the method parameter type to `ChronoUnit` allows you to omit the
explicit enum type from the annotation as follows.

```
@ParameterizedTest
@EnumSource
void testWithEnumSourceWithAutoDetection(ChronoUnit unit) {
    assertNotNull(unit);
}
```

The annotation provides an optional `names` attribute that lets you specify which
constants shall be used, like in the following example. If omitted, all constants will be
used.

```
@ParameterizedTest
@EnumSource(names = { "DAYS", "HOURS" })
void testWithEnumSourceInclude(ChronoUnit unit) {
    assertTrue(EnumSet.of(ChronoUnit.DAYS, ChronoUnit.HOURS).contains(unit));
}
```

The `@EnumSource` annotation also provides an optional `mode` attribute that enables
fine-grained control over which constants are passed to the test method. For example, you
can exclude names from the enum constant pool or specify regular expressions as in the
following examples.

```
@ParameterizedTest
@EnumSource(mode = EXCLUDE, names = { "ERAS", "FOREVER" })
void testWithEnumSourceExclude(ChronoUnit unit) {
    assertFalse(EnumSet.of(ChronoUnit.ERAS, ChronoUnit.FOREVER).contains(unit));
}
```

```
@ParameterizedTest
@EnumSource(mode = MATCH_ALL, names = "^.*DAYS$")
void testWithEnumSourceRegex(ChronoUnit unit) {
    assertTrue(unit.name().endsWith("DAYS"));
}
```

##### [](#writing-tests-parameterized-tests-sources-MethodSource)@MethodSource #####

`[@MethodSource](../api/org.junit.jupiter.params/org/junit/jupiter/params/provider/MethodSource.html)` allows you to refer to one or more *factory* methods of the test class
or external classes.

Factory methods within the test class must be `static` unless the test class is annotated
with `@TestInstance(Lifecycle.PER_CLASS)`; whereas, factory methods in external classes
must always be `static`.

Each factory method must generate a *stream* of *arguments*, and each set of arguments
within the stream will be provided as the physical arguments for individual invocations
of the annotated `@ParameterizedTest` method. Generally speaking this translates to a`Stream` of `Arguments` (i.e., `Stream<Arguments>`); however, the actual concrete return
type can take on many forms. In this context, a "stream" is anything that JUnit can
reliably convert into a `Stream`, such as `Stream`, `DoubleStream`, `LongStream`,`IntStream`, `Collection`, `Iterator`, `Iterable`, an array of objects, or an array of
primitives. The "arguments" within the stream can be supplied as an instance of`Arguments`, an array of objects (e.g., `Object[]`), or a single value if the
parameterized test method accepts a single argument.

If you only need a single parameter, you can return a `Stream` of instances of the
parameter type as demonstrated in the following example.

```
@ParameterizedTest
@MethodSource("stringProvider")
void testWithExplicitLocalMethodSource(String argument) {
    assertNotNull(argument);
}

static Stream<String> stringProvider() {
    return Stream.of("apple", "banana");
}
```

If you do not explicitly provide a factory method name via `@MethodSource`, JUnit Jupiter
will search for a *factory* method that has the same name as the current`@ParameterizedTest` method by convention. This is demonstrated in the following example.

```
@ParameterizedTest
@MethodSource
void testWithDefaultLocalMethodSource(String argument) {
    assertNotNull(argument);
}

static Stream<String> testWithDefaultLocalMethodSource() {
    return Stream.of("apple", "banana");
}
```

Streams for primitive types (`DoubleStream`, `IntStream`, and `LongStream`) are also
supported as demonstrated by the following example.

```
@ParameterizedTest
@MethodSource("range")
void testWithRangeMethodSource(int argument) {
    assertNotEquals(9, argument);
}

static IntStream range() {
    return IntStream.range(0, 20).skip(10);
}
```

If a parameterized test method declares multiple parameters, you need to return a
collection, stream, or array of `Arguments` instances or object arrays as shown below
(see the Javadoc for `[@MethodSource](../api/org.junit.jupiter.params/org/junit/jupiter/params/provider/MethodSource.html)` for further details on supported return types).
Note that `arguments(Object…​)` is a static factory method defined in the `Arguments`interface. In addition, `Arguments.of(Object…​)` may be used as an alternative to`arguments(Object…​)`.

```
@ParameterizedTest
@MethodSource("stringIntAndListProvider")
void testWithMultiArgMethodSource(String str, int num, List<String> list) {
    assertEquals(5, str.length());
    assertTrue(num >=1 && num <=2);
    assertEquals(2, list.size());
}

static Stream<Arguments> stringIntAndListProvider() {
    return Stream.of(
        arguments("apple", 1, Arrays.asList("a", "b")),
        arguments("lemon", 2, Arrays.asList("x", "y"))
    );
}
```

An external, `static` *factory* method can be referenced by providing its *fully qualified
method name* as demonstrated in the following example.

```
package example;

import java.util.stream.Stream;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.MethodSource;

class ExternalMethodSourceDemo {

    @ParameterizedTest
    @MethodSource("example.StringsProviders#tinyStrings")
    void testWithExternalMethodSource(String tinyString) {
        // test with tiny string
    }
}

class StringsProviders {

    static Stream<String> tinyStrings() {
        return Stream.of(".", "oo", "OOO");
    }
}
```

Factory methods can declare parameters, which will be provided by registered
implementations of the `ParameterResolver` extension API. In the following example, the
factory method is referenced by its name since there is only one such method in the test
class. If there are several local methods with the same name, parameters can also be
provided to differentiate them – for example, `@MethodSource("factoryMethod()")` or`@MethodSource("factoryMethod(java.lang.String)")`. Alternatively, the factory method
can be referenced by its fully qualified method name, e.g.`@MethodSource("example.MyTests#factoryMethod(java.lang.String)")`.

```
@RegisterExtension
static final IntegerResolver integerResolver = new IntegerResolver();

@ParameterizedTest
@MethodSource("factoryMethodWithArguments")
void testWithFactoryMethodWithArguments(String argument) {
    assertTrue(argument.startsWith("2"));
}

static Stream<Arguments> factoryMethodWithArguments(int quantity) {
    return Stream.of(
            arguments(quantity + " apples"),
            arguments(quantity + " lemons")
    );
}

static class IntegerResolver implements ParameterResolver {

    @Override
    public boolean supportsParameter(ParameterContext parameterContext,
            ExtensionContext extensionContext) {

        return parameterContext.getParameter().getType() == int.class;
    }

    @Override
    public Object resolveParameter(ParameterContext parameterContext,
            ExtensionContext extensionContext) {

        return 2;
    }

}
```

##### [](#writing-tests-parameterized-tests-sources-CsvSource)@CsvSource #####

`@CsvSource` allows you to express argument lists as comma-separated values (i.e., CSV`String` literals). Each string provided via the `value` attribute in `@CsvSource`represents a CSV record and results in one invocation of the parameterized test. The first
record may optionally be used to supply CSV headers (see the Javadoc for the`useHeadersInDisplayName` attribute for details and an example).

```
@ParameterizedTest
@CsvSource({
    "apple,         1",
    "banana,        2",
    "'lemon, lime', 0xF1",
    "strawberry,    700_000"
})
void testWithCsvSource(String fruit, int rank) {
    assertNotNull(fruit);
    assertNotEquals(0, rank);
}
```

The default delimiter is a comma (`,`), but you can use another character by setting the`delimiter` attribute. Alternatively, the `delimiterString` attribute allows you to use a`String` delimiter instead of a single character. However, both delimiter attributes
cannot be set simultaneously.

By default, `@CsvSource` uses a single quote (`'`) as its quote character, but this can be
changed via the `quoteCharacter` attribute. See the `'lemon, lime'` value in the example
above and in the table below. An empty, quoted value (`''`) results in an empty `String`unless the `emptyValue` attribute is set; whereas, an entirely *empty* value is
interpreted as a `null` reference. By specifying one or more `nullValues`, a custom value
can be interpreted as a `null` reference (see the `NIL` example in the table below). An`ArgumentConversionException` is thrown if the target type of a `null` reference is a
primitive type.

|   |An *unquoted* empty value will always be converted to a `null` reference regardless<br/>of any custom values configured via the `nullValues` attribute.|
|---|-------------------------------------------------------------------------------------------------------------------------------------------------------|

Except within a quoted string, leading and trailing whitespace in a CSV column is trimmed
by default. This behavior can be changed by setting the`ignoreLeadingAndTrailingWhitespace` attribute to `true`.

|                                     Example Input                                     |   Resulting Argument List   |
|---------------------------------------------------------------------------------------|-----------------------------|
|                           `@CsvSource({ "apple, banana" })`                           |    `"apple"`, `"banana"`    |
|                       `@CsvSource({ "apple, 'lemon, lime'" })`                        | `"apple"`, `"lemon, lime"`  |
|                             `@CsvSource({ "apple, ''" })`                             |       `"apple"`, `""`       |
|                              `@CsvSource({ "apple, " })`                              |      `"apple"`, `null`      |
|          `@CsvSource(value = { "apple, banana, NIL" }, nullValues = "NIL")`           |`"apple"`, `"banana"`, `null`|
|`@CsvSource(value = { " apple , banana" }, ignoreLeadingAndTrailingWhitespace = false)`|  `" apple "`, `" banana"`   |

If the programming language you are using supports *text blocks* — for example, Java SE
15 or higher — you can alternatively use the `textBlock` attribute of `@CsvSource`. Each
record within a text block represents a CSV record and results in one invocation of the
parameterized test. The first record may optionally be used to supply CSV headers by
setting the `useHeadersInDisplayName` attribute to `true` as in the example below.

Using a text block, the previous example can be implemented as follows.

```
@ParameterizedTest(name = "[{index}] {arguments}")
@CsvSource(useHeadersInDisplayName = true, textBlock = """
    FRUIT,         RANK
    apple,         1
    banana,        2
    'lemon, lime', 0xF1
    strawberry,    700_000
    """)
void testWithCsvSource(String fruit, int rank) {
    // ...
}
```

The generated display names for the previous example include the CSV header names.

```
[1] FRUIT = apple, RANK = 1
[2] FRUIT = banana, RANK = 2
[3] FRUIT = lemon, lime, RANK = 0xF1
[4] FRUIT = strawberry, RANK = 700_000
```

In contrast to CSV records supplied via the `value` attribute, a text block can contain
comments. Any line beginning with a `#` symbol will be treated as a comment and
ignored. Note, however, that the `#` symbol must be the first character on the line
without any leading whitespace. It is therefore recommended that the closing text block
delimiter (`"""`) be placed either at the end of the last line of input or on the
following line, left aligned with the rest of the input (as can be seen in the example
below which demonstrates formatting similar to a table).

```
@ParameterizedTest
@CsvSource(delimiter = '|', quoteCharacter = '"', textBlock = """
    #-----------------------------
    #    FRUIT     |     RANK
    #-----------------------------
         apple     |      1
    #-----------------------------
         banana    |      2
    #-----------------------------
      "lemon lime" |     0xF1
    #-----------------------------
       strawberry  |    700_000
    #-----------------------------
    """)
void testWithCsvSource(String fruit, int rank) {
    // ...
}
```

|   |Java’s [text block](https://docs.oracle.com/en/java/javase/15/text-blocks/index.html)feature automatically removes *incidental whitespace* when the code is compiled.<br/>However other JVM languages such as Groovy and Kotlin do not. Thus, if you are using a<br/>programming language other than Java and your text block contains comments or new lines<br/>within quoted strings, you will need to ensure that there is no leading whitespace within<br/>your text block.|
|---|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

##### [](#writing-tests-parameterized-tests-sources-CsvFileSource)@CsvFileSource #####

`@CsvFileSource` lets you use comma-separated value (CSV) files from the classpath or the
local file system. Each record from a CSV file results in one invocation of the
parameterized test. The first record may optionally be used to supply CSV headers. You can
instruct JUnit to ignore the headers via the `numLinesToSkip` attribute. If you would like
for the headers to be used in the display names, you can set the `useHeadersInDisplayName`attribute to `true`. The examples below demonstrate the use of `numLinesToSkip` and`useHeadersInDisplayName`.

The default delimiter is a comma (`,`), but you can use another character by setting the`delimiter` attribute. Alternatively, the `delimiterString` attribute allows you to use a`String` delimiter instead of a single character. However, both delimiter attributes
cannot be set simultaneously.

|   |Comments in CSV files<br/><br/>Any line beginning with a `#` symbol will be interpreted as a comment and will<br/>be ignored.|
|---|-----------------------------------------------------------------------------------------------------------------------------|

```
@ParameterizedTest
@CsvFileSource(resources = "/two-column.csv", numLinesToSkip = 1)
void testWithCsvFileSourceFromClasspath(String country, int reference) {
    assertNotNull(country);
    assertNotEquals(0, reference);
}

@ParameterizedTest
@CsvFileSource(files = "src/test/resources/two-column.csv", numLinesToSkip = 1)
void testWithCsvFileSourceFromFile(String country, int reference) {
    assertNotNull(country);
    assertNotEquals(0, reference);
}

@ParameterizedTest(name = "[{index}] {arguments}")
@CsvFileSource(resources = "/two-column.csv", useHeadersInDisplayName = true)
void testWithCsvFileSourceAndHeaders(String country, int reference) {
    assertNotNull(country);
    assertNotEquals(0, reference);
}
```

two-column.csv

```
COUNTRY, REFERENCE
Sweden, 1
Poland, 2
"United States of America", 3
France, 700_000
```

The following listing shows the generated display names for the first two parameterized
test methods above.

```
[1] country=Sweden, reference=1
[2] country=Poland, reference=2
[3] country=United States of America, reference=3
[4] country=France, reference=700_000
```

The following listing shows the generated display names for the last parameterized test
method above that uses CSV header names.

```
[1] COUNTRY = Sweden, REFERENCE = 1
[2] COUNTRY = Poland, REFERENCE = 2
[3] COUNTRY = United States of America, REFERENCE = 3
[4] COUNTRY = France, REFERENCE = 700_000
```

In contrast to the default syntax used in `@CsvSource`, `@CsvFileSource` uses a double
quote (`"`) as the quote character by default, but this can be changed via the`quoteCharacter` attribute. See the `"United States of America"` value in the example
above. An empty, quoted value (`""`) results in an empty `String` unless the`emptyValue` attribute is set; whereas, an entirely *empty* value is interpreted as a`null` reference. By specifying one or more `nullValues`, a custom value can be
interpreted as a `null` reference. An `ArgumentConversionException` is thrown if the
target type of a `null` reference is a primitive type.

|   |An *unquoted* empty value will always be converted to a `null` reference regardless<br/>of any custom values configured via the `nullValues` attribute.|
|---|-------------------------------------------------------------------------------------------------------------------------------------------------------|

Except within a quoted string, leading and trailing whitespace in a CSV column is trimmed
by default. This behavior can be changed by setting the`ignoreLeadingAndTrailingWhitespace` attribute to `true`.

##### [](#writing-tests-parameterized-tests-sources-ArgumentsSource)@ArgumentsSource #####

`@ArgumentsSource` can be used to specify a custom, reusable `ArgumentsProvider`. Note
that an implementation of `ArgumentsProvider` must be declared as either a top-level
class or as a `static` nested class.

```
@ParameterizedTest
@ArgumentsSource(MyArgumentsProvider.class)
void testWithArgumentsSource(String argument) {
    assertNotNull(argument);
}
```

```
public class MyArgumentsProvider implements ArgumentsProvider {

    @Override
    public Stream<? extends Arguments> provideArguments(ExtensionContext context) {
        return Stream.of("apple", "banana").map(Arguments::of);
    }
}
```

If you wish to implement a custom `ArgumentsProvider` that also consumes an annotation
(like built-in providers such as `[ValueArgumentsProvider](../api/org.junit.jupiter.params/org/junit/jupiter/params/provider/ValueArgumentsProvider.html)` or `[CsvArgumentsProvider](../api/org.junit.jupiter.params/org/junit/jupiter/params/provider/CsvArgumentsProvider.html)`),
you have the possibility to extend the `[AnnotationBasedArgumentsProvider](../api/org.junit.jupiter.params/org/junit/jupiter/params/provider/AnnotationBasedArgumentsProvider.html)` class.

#### [](#writing-tests-parameterized-tests-argument-conversion)2.16.4. Argument Conversion ####

##### [](#writing-tests-parameterized-tests-argument-conversion-widening)Widening Conversion #####

JUnit Jupiter supports[Widening Primitive
Conversion](https://docs.oracle.com/javase/specs/jls/se8/html/jls-5.html#jls-5.1.2) for arguments supplied to a `@ParameterizedTest`. For example, a
parameterized test annotated with `@ValueSource(ints = { 1, 2, 3 })` can be declared to
accept not only an argument of type `int` but also an argument of type `long`, `float`,
or `double`.

##### [](#writing-tests-parameterized-tests-argument-conversion-implicit)Implicit Conversion #####

To support use cases like `@CsvSource`, JUnit Jupiter provides a number of built-in
implicit type converters. The conversion process depends on the declared type of each
method parameter.

For example, if a `@ParameterizedTest` declares a parameter of type `TimeUnit` and the
actual type supplied by the declared source is a `String`, the string will be
automatically converted into the corresponding `TimeUnit` enum constant.

```
@ParameterizedTest
@ValueSource(strings = "SECONDS")
void testWithImplicitArgumentConversion(ChronoUnit argument) {
    assertNotNull(argument.name());
}
```

`String` instances are implicitly converted to the following target types.

|   |Decimal, hexadecimal, and octal `String` literals will be converted to their<br/>integral types: `byte`, `short`, `int`, `long`, and their boxed counterparts.|
|---|--------------------------------------------------------------------------------------------------------------------------------------------------------------|

|       Target Type        |                                                     Example                                                     |
|--------------------------|-----------------------------------------------------------------------------------------------------------------|
|   `boolean`/`Boolean`    |                  `"true"` → `true` *(only accepts values 'true' or 'false', case-insensitive)*                  |
|      `byte`/`Byte`       |                                    `"15"`, `"0xF"`, or `"017"` → `(byte) 15`                                    |
|    `char`/`Character`    |                                                  `"o"` → `'o'`                                                  |
|     `short`/`Short`      |                                   `"15"`, `"0xF"`, or `"017"` → `(short) 15`                                    |
|     `int`/`Integer`      |                                       `"15"`, `"0xF"`, or `"017"` → `15`                                        |
|      `long`/`Long`       |                                       `"15"`, `"0xF"`, or `"017"` → `15L`                                       |
|     `float`/`Float`      |                                                `"1.0"` → `1.0f`                                                 |
|    `double`/`Double`     |                                                `"1.0"` → `1.0d`                                                 |
|     `Enum` subclass      |                                        `"SECONDS"` → `TimeUnit.SECONDS`                                         |
|      `java.io.File`      |                                 `"/path/to/file"` → `new File("/path/to/file")`                                 |
|    `java.lang.Class`     |`"java.lang.Integer"` → `java.lang.Integer.class` *(use `$` for nested classes, e.g. `"java.lang.Thread$State"`)*|
|    `java.lang.Class`     |                            `"byte"` → `byte.class` *(primitive types are supported)*                            |
|    `java.lang.Class`     |                            `"char[]"` → `char[].class` *(array types are supported)*                            |
|  `java.math.BigDecimal`  |                                `"123.456e789"` → `new BigDecimal("123.456e789")`                                |
|  `java.math.BigInteger`  |                        `"1234567890123456789"` → `new BigInteger("1234567890123456789")`                        |
|      `java.net.URI`      |                           `"https://junit.org/"` → `URI.create("https://junit.org/")`                           |
|      `java.net.URL`      |                       `"https://junit.org/"` → `URI.create("https://junit.org/").toURL()`                       |
|`java.nio.charset.Charset`|                                     `"UTF-8"` → `Charset.forName("UTF-8")`                                      |
|   `java.nio.file.Path`   |                                `"/path/to/file"` → `Paths.get("/path/to/file")`                                 |
|   `java.time.Duration`   |                                       `"PT3S"` → `Duration.ofSeconds(3)`                                        |
|   `java.time.Instant`    |                              `"1970-01-01T00:00:00Z"` → `Instant.ofEpochMilli(0)`                               |
|`java.time.LocalDateTime` |             `"2017-03-14T12:34:56.789"` → `LocalDateTime.of(2017, 3, 14, 12, 34, 56, 789_000_000)`              |
|  `java.time.LocalDate`   |                                  `"2017-03-14"` → `LocalDate.of(2017, 3, 14)`                                   |
|  `java.time.LocalTime`   |                           `"12:34:56.789"` → `LocalTime.of(12, 34, 56, 789_000_000)`                            |
|   `java.time.MonthDay`   |                                       `"--03-14"` → `MonthDay.of(3, 14)`                                        |
|`java.time.OffsetDateTime`|    `"2017-03-14T12:34:56.789Z"` → `OffsetDateTime.of(2017, 3, 14, 12, 34, 56, 789_000_000, ZoneOffset.UTC)`     |
|  `java.time.OffsetTime`  |                  `"12:34:56.789Z"` → `OffsetTime.of(12, 34, 56, 789_000_000, ZoneOffset.UTC)`                   |
|    `java.time.Period`    |                                        `"P2M6D"` → `Period.of(0, 2, 6)`                                         |
|  `java.time.YearMonth`   |                                      `"2017-03"` → `YearMonth.of(2017, 3)`                                      |
|     `java.time.Year`     |                                           `"2017"` → `Year.of(2017)`                                            |
|`java.time.ZonedDateTime` |     `"2017-03-14T12:34:56.789Z"` → `ZonedDateTime.of(2017, 3, 14, 12, 34, 56, 789_000_000, ZoneOffset.UTC)`     |
|    `java.time.ZoneId`    |                                `"Europe/Berlin"` → `ZoneId.of("Europe/Berlin")`                                 |
|  `java.time.ZoneOffset`  |                                 `"+02:30"` → `ZoneOffset.ofHoursMinutes(2, 30)`                                 |
|   `java.util.Currency`   |                                     `"JPY"` → `Currency.getInstance("JPY")`                                     |
|    `java.util.Locale`    |                                           `"en"` → `new Locale("en")`                                           |
|     `java.util.UUID`     |      `"d043e930-7b3b-48e3-bdbe-5a3ccfb833db"` → `UUID.fromString("d043e930-7b3b-48e3-bdbe-5a3ccfb833db")`       |

###### [](#writing-tests-parameterized-tests-argument-conversion-implicit-fallback)Fallback String-to-Object Conversion ######

In addition to implicit conversion from strings to the target types listed in the above
table, JUnit Jupiter also provides a fallback mechanism for automatic conversion from a`String` to a given target type if the target type declares exactly one suitable *factory
method* or a *factory constructor* as defined below.

* *factory method*: a non-private, `static` method declared in the target type that
  accepts a single `String` argument and returns an instance of the target type. The name
  of the method can be arbitrary and need not follow any particular convention.

* *factory constructor*: a non-private constructor in the target type that accepts a
  single `String` argument. Note that the target type must be declared as either a
  top-level class or as a `static` nested class.

|   |If multiple *factory methods* are discovered, they will be ignored. If a *factory<br/>method* and a *factory constructor* are discovered, the factory method will be used<br/>instead of the constructor.|
|---|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

For example, in the following `@ParameterizedTest` method, the `Book` argument will be
created by invoking the `Book.fromTitle(String)` factory method and passing `"42 Cats"`as the title of the book.

```
@ParameterizedTest
@ValueSource(strings = "42 Cats")
void testWithImplicitFallbackArgumentConversion(Book book) {
    assertEquals("42 Cats", book.getTitle());
}
```

```
public class Book {

    private final String title;

    private Book(String title) {
        this.title = title;
    }

    public static Book fromTitle(String title) {
        return new Book(title);
    }

    public String getTitle() {
        return this.title;
    }
}
```

##### [](#writing-tests-parameterized-tests-argument-conversion-explicit)Explicit Conversion #####

Instead of relying on implicit argument conversion you may explicitly specify an`ArgumentConverter` to use for a certain parameter using the `@ConvertWith` annotation
like in the following example. Note that an implementation of `ArgumentConverter` must be
declared as either a top-level class or as a `static` nested class.

```
@ParameterizedTest
@EnumSource(ChronoUnit.class)
void testWithExplicitArgumentConversion(
        @ConvertWith(ToStringArgumentConverter.class) String argument) {

    assertNotNull(ChronoUnit.valueOf(argument));
}
```

```
public class ToStringArgumentConverter extends SimpleArgumentConverter {

    @Override
    protected Object convert(Object source, Class<?> targetType) {
        assertEquals(String.class, targetType, "Can only convert to String");
        if (source instanceof Enum<?>) {
            return ((Enum<?>) source).name();
        }
        return String.valueOf(source);
    }
}
```

If the converter is only meant to convert one type to another, you can extend`TypedArgumentConverter` to avoid boilerplate type checks.

```
public class ToLengthArgumentConverter extends TypedArgumentConverter<String, Integer> {

    protected ToLengthArgumentConverter() {
        super(String.class, Integer.class);
    }

    @Override
    protected Integer convert(String source) {
        return (source != null ? source.length() : 0);
    }

}
```

Explicit argument converters are meant to be implemented by test and extension authors.
Thus, `junit-jupiter-params` only provides a single explicit argument converter that may
also serve as a reference implementation: `JavaTimeArgumentConverter`. It is used via the
composed annotation `JavaTimeConversionPattern`.

```
@ParameterizedTest
@ValueSource(strings = { "01.01.2017", "31.12.2017" })
void testWithExplicitJavaTimeConverter(
        @JavaTimeConversionPattern("dd.MM.yyyy") LocalDate argument) {

    assertEquals(2017, argument.getYear());
}
```

If you wish to implement a custom `ArgumentConverter` that also consumes an annotation
(like `JavaTimeArgumentConverter`), you have the possibility to extend the`[AnnotationBasedArgumentConverter](../api/org.junit.jupiter.params/org/junit/jupiter/params/provider/AnnotationBasedArgumentConverter.html)` class.

#### [](#writing-tests-parameterized-tests-argument-aggregation)2.16.5. Argument Aggregation ####

By default, each *argument* provided to a `@ParameterizedTest` method corresponds to a
single method parameter. Consequently, argument sources which are expected to supply a
large number of arguments can lead to large method signatures.

In such cases, an `[ArgumentsAccessor](../api/org.junit.jupiter.params/org/junit/jupiter/params/aggregator/ArgumentsAccessor.html)` can be used instead of multiple parameters. Using
this API, you can access the provided arguments through a single argument passed to your
test method. In addition, type conversion is supported as discussed in[Implicit Conversion](#writing-tests-parameterized-tests-argument-conversion-implicit).

Besides, you can retrieve the current test invocation index with`ArgumentsAccessor.getInvocationIndex()`.

```
@ParameterizedTest
@CsvSource({
    "Jane, Doe, F, 1990-05-20",
    "John, Doe, M, 1990-10-22"
})
void testWithArgumentsAccessor(ArgumentsAccessor arguments) {
    Person person = new Person(arguments.getString(0),
                               arguments.getString(1),
                               arguments.get(2, Gender.class),
                               arguments.get(3, LocalDate.class));

    if (person.getFirstName().equals("Jane")) {
        assertEquals(Gender.F, person.getGender());
    }
    else {
        assertEquals(Gender.M, person.getGender());
    }
    assertEquals("Doe", person.getLastName());
    assertEquals(1990, person.getDateOfBirth().getYear());
}
```

*An instance of `ArgumentsAccessor` is automatically injected into any parameter of type`ArgumentsAccessor`.*

##### [](#writing-tests-parameterized-tests-argument-aggregation-custom)Custom Aggregators #####

Apart from direct access to a `@ParameterizedTest` method’s arguments using an`ArgumentsAccessor`, JUnit Jupiter also supports the usage of custom, reusable*aggregators*.

To use a custom aggregator, implement the `[ArgumentsAggregator](../api/org.junit.jupiter.params/org/junit/jupiter/params/aggregator/ArgumentsAggregator.html)` interface and register
it via the `@AggregateWith` annotation on a compatible parameter in the`@ParameterizedTest` method. The result of the aggregation will then be provided as an
argument for the corresponding parameter when the parameterized test is invoked. Note
that an implementation of `ArgumentsAggregator` must be declared as either a top-level
class or as a `static` nested class.

```
@ParameterizedTest
@CsvSource({
    "Jane, Doe, F, 1990-05-20",
    "John, Doe, M, 1990-10-22"
})
void testWithArgumentsAggregator(@AggregateWith(PersonAggregator.class) Person person) {
    // perform assertions against person
}
```

```
public class PersonAggregator implements ArgumentsAggregator {
    @Override
    public Person aggregateArguments(ArgumentsAccessor arguments, ParameterContext context) {
        return new Person(arguments.getString(0),
                          arguments.getString(1),
                          arguments.get(2, Gender.class),
                          arguments.get(3, LocalDate.class));
    }
}
```

If you find yourself repeatedly declaring `@AggregateWith(MyTypeAggregator.class)` for
multiple parameterized test methods across your codebase, you may wish to create a custom*composed annotation* such as `@CsvToMyType` that is meta-annotated with`@AggregateWith(MyTypeAggregator.class)`. The following example demonstrates this in
action with a custom `@CsvToPerson` annotation.

```
@ParameterizedTest
@CsvSource({
    "Jane, Doe, F, 1990-05-20",
    "John, Doe, M, 1990-10-22"
})
void testWithCustomAggregatorAnnotation(@CsvToPerson Person person) {
    // perform assertions against person
}
```

```
@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.PARAMETER)
@AggregateWith(PersonAggregator.class)
public @interface CsvToPerson {
}
```

#### [](#writing-tests-parameterized-tests-display-names)2.16.6. Customizing Display Names ####

By default, the display name of a parameterized test invocation contains the invocation
index and the `String` representation of all arguments for that specific invocation.
Each of them is preceded by the parameter name (unless the argument is only available via
an `ArgumentsAccessor` or `ArgumentAggregator`), if present in the bytecode (for Java,
test code must be compiled with the `-parameters` compiler flag).

However, you can customize invocation display names via the `name` attribute of the`@ParameterizedTest` annotation like in the following example.

```
@DisplayName("Display name of container")
@ParameterizedTest(name = "{index} ==> the rank of ''{0}'' is {1}")
@CsvSource({ "apple, 1", "banana, 2", "'lemon, lime', 3" })
void testWithCustomDisplayNames(String fruit, int rank) {
}
```

When executing the above method using the `ConsoleLauncher` you will see output similar to
the following.

```
Display name of container ✔
├─ 1 ==> the rank of 'apple' is 1 ✔
├─ 2 ==> the rank of 'banana' is 2 ✔
└─ 3 ==> the rank of 'lemon, lime' is 3 ✔
```

Please note that `name` is a `MessageFormat` pattern. Thus, a single quote (`'`) needs to
be represented as a doubled single quote (`''`) in order to be displayed.

The following placeholders are supported within custom display names.

|     Placeholder      |                           Description                           |
|----------------------|-----------------------------------------------------------------|
|   `{displayName}`    |                 the display name of the method                  |
|      `{index}`       |             the current invocation index (1-based)              |
|    `{arguments}`     |          the complete, comma-separated arguments list           |
|`{argumentsWithNames}`|the complete, comma-separated arguments list with parameter names|
|   `{0}`, `{1}`, …​   |                     an individual argument                      |

|   |When including arguments in display names, their string representations are truncated<br/>if they exceed the configured maximum length. The limit is configurable via the`junit.jupiter.params.displayname.argument.maxlength` configuration parameter and defaults<br/>to 512 characters.|
|---|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

When using `@MethodSource` or `@ArgumentsSource`, you can provide custom names for
arguments using the `[Named](../api/org.junit.jupiter.api/org/junit/jupiter/api/Named.html)` API. A custom name will be used if the argument is included
in the invocation display name, like in the example below.

```
@DisplayName("A parameterized test with named arguments")
@ParameterizedTest(name = "{index}: {0}")
@MethodSource("namedArguments")
void testWithNamedArguments(File file) {
}

static Stream<Arguments> namedArguments() {
    return Stream.of(
        arguments(named("An important file", new File("path1"))),
        arguments(named("Another file", new File("path2")))
    );
}
```

```
A parameterized test with named arguments ✔
├─ 1: An important file ✔
└─ 2: Another file ✔
```

If you’d like to set a default name pattern for all parameterized tests in your project,
you can declare the `junit.jupiter.params.displayname.default` configuration parameter in
the `junit-platform.properties` file as demonstrated in the following example (see[Configuration Parameters](#running-tests-config-params) for other options).

```
junit.jupiter.params.displayname.default = {index}
```

The display name for a parameterized test is determined according to the following
precedence rules:

1. `name` attribute in `@ParameterizedTest`, if present

2. value of the `junit.jupiter.params.displayname.default` configuration parameter, if present

3. `DEFAULT_DISPLAY_NAME` constant defined in `@ParameterizedTest`

#### [](#writing-tests-parameterized-tests-lifecycle-interop)2.16.7. Lifecycle and Interoperability ####

Each invocation of a parameterized test has the same lifecycle as a regular `@Test`method. For example, `@BeforeEach` methods will be executed before each invocation.
Similar to [Dynamic Tests](#writing-tests-dynamic-tests), invocations will appear one by one in the
test tree of an IDE. You may at will mix regular `@Test` methods and `@ParameterizedTest`methods within the same test class.

You may use `ParameterResolver` extensions with `@ParameterizedTest` methods. However,
method parameters that are resolved by argument sources need to come first in the
argument list. Since a test class may contain regular tests as well as parameterized
tests with different parameter lists, values from argument sources are not resolved for
lifecycle methods (e.g. `@BeforeEach`) and test class constructors.

```
@BeforeEach
void beforeEach(TestInfo testInfo) {
    // ...
}

@ParameterizedTest
@ValueSource(strings = "apple")
void testWithRegularParameterResolver(String argument, TestReporter testReporter) {
    testReporter.publishEntry("argument", argument);
}

@AfterEach
void afterEach(TestInfo testInfo) {
    // ...
}
```

### [](#writing-tests-test-templates)2.17. Test Templates ###

A `[@TestTemplate](../api/org.junit.jupiter.api/org/junit/jupiter/api/TestTemplate.html)` method is not a regular test case but rather a template for test
cases. As such, it is designed to be invoked multiple times depending on the number of
invocation contexts returned by the registered providers. Thus, it must be used in
conjunction with a registered `[TestTemplateInvocationContextProvider](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/TestTemplateInvocationContextProvider.html)` extension. Each
invocation of a test template method behaves like the execution of a regular `@Test`method with full support for the same lifecycle callbacks and extensions. Please refer to[Providing Invocation Contexts for Test Templates](#extensions-test-templates) for usage examples.

|   |[Repeated Tests](#writing-tests-repeated-tests) and [Parameterized Tests](#writing-tests-parameterized-tests) are<br/>built-in specializations of test templates.|
|---|-----------------------------------------------------------------------------------------------------------------------------------------------------------------|

### [](#writing-tests-dynamic-tests)2.18. Dynamic Tests ###

The standard `@Test` annotation in JUnit Jupiter described in[Annotations](#writing-tests-annotations) is very similar to the `@Test` annotation in JUnit 4. Both
describe methods that implement test cases. These test cases are static in the sense that
they are fully specified at compile time, and their behavior cannot be changed by
anything happening at runtime. *Assumptions provide a basic form of dynamic behavior but
are intentionally rather limited in their expressiveness.*

In addition to these standard tests a completely new kind of test programming model has
been introduced in JUnit Jupiter. This new kind of test is a *dynamic test* which is
generated at runtime by a factory method that is annotated with `@TestFactory`.

In contrast to `@Test` methods, a `@TestFactory` method is not itself a test case but
rather a factory for test cases. Thus, a dynamic test is the product of a factory.
Technically speaking, a `@TestFactory` method must return a single `DynamicNode` or a`Stream`, `Collection`, `Iterable`, `Iterator`, or array of `DynamicNode` instances.
Instantiable subclasses of `DynamicNode` are `DynamicContainer` and `DynamicTest`.`DynamicContainer` instances are composed of a *display name* and a list of dynamic child
nodes, enabling the creation of arbitrarily nested hierarchies of dynamic nodes.`DynamicTest` instances will be executed lazily, enabling dynamic and even
non-deterministic generation of test cases.

Any `Stream` returned by a `@TestFactory` will be properly closed by calling`stream.close()`, making it safe to use a resource such as `Files.lines()`.

As with `@Test` methods, `@TestFactory` methods must not be `private` or `static` and may
optionally declare parameters to be resolved by `ParameterResolvers`.

A `DynamicTest` is a test case generated at runtime. It is composed of a *display name*and an `Executable`. `Executable` is a `@FunctionalInterface` which means that the
implementations of dynamic tests can be provided as *lambda expressions* or *method
references*.

|   |Dynamic Test Lifecycle<br/><br/>The execution lifecycle of a dynamic test is quite different than it is for a<br/>standard `@Test` case. Specifically, there are no lifecycle callbacks for individual<br/>dynamic tests. This means that `@BeforeEach` and `@AfterEach` methods and their<br/>corresponding extension callbacks are executed for the `@TestFactory` method but not for<br/>each *dynamic test*. In other words, if you access fields from the test instance within a<br/>lambda expression for a dynamic test, those fields will not be reset by callback methods<br/>or extensions between the execution of individual dynamic tests generated by the same`@TestFactory` method.|
|---|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

As of JUnit Jupiter 5.10.2, dynamic tests must always be created by factory
methods; however, this might be complemented by a registration facility in a later
release.

#### [](#writing-tests-dynamic-tests-examples)2.18.1. Dynamic Test Examples ####

The following `DynamicTestsDemo` class demonstrates several examples of test factories
and dynamic tests.

The first method returns an invalid return type. Since an invalid return type cannot be
detected at compile time, a `JUnitException` is thrown when it is detected at runtime.

The next six methods demonstrate the generation of a `Collection`, `Iterable`, `Iterator`,
array, or `Stream` of `DynamicTest` instances. Most of these examples do not really
exhibit dynamic behavior but merely demonstrate the supported return types in principle.
However, `dynamicTestsFromStream()` and `dynamicTestsFromIntStream()` demonstrate how to
generate dynamic tests for a given set of strings or a range of input numbers.

The next method is truly dynamic in nature. `generateRandomNumberOfTests()` implements an`Iterator` that generates random numbers, a display name generator, and a test executor
and then provides all three to `DynamicTest.stream()`. Although the non-deterministic
behavior of `generateRandomNumberOfTests()` is of course in conflict with test
repeatability and should thus be used with care, it serves to demonstrate the
expressiveness and power of dynamic tests.

The next method is similar to `generateRandomNumberOfTests()` in terms of flexibility;
however, `dynamicTestsFromStreamFactoryMethod()` generates a stream of dynamic tests from
an existing `Stream` via the `DynamicTest.stream()` factory method.

For demonstration purposes, the `dynamicNodeSingleTest()` method generates a single`DynamicTest` instead of a stream, and the `dynamicNodeSingleContainer()` method generates
a nested hierarchy of dynamic tests utilizing `DynamicContainer`.

```
import static example.util.StringUtils.isPalindrome;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.DynamicContainer.dynamicContainer;
import static org.junit.jupiter.api.DynamicTest.dynamicTest;
import static org.junit.jupiter.api.Named.named;

import java.util.Arrays;
import java.util.Collection;
import java.util.Iterator;
import java.util.List;
import java.util.Random;
import java.util.function.Function;
import java.util.stream.IntStream;
import java.util.stream.Stream;

import example.util.Calculator;

import org.junit.jupiter.api.DynamicNode;
import org.junit.jupiter.api.DynamicTest;
import org.junit.jupiter.api.Named;
import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.TestFactory;
import org.junit.jupiter.api.function.ThrowingConsumer;

class DynamicTestsDemo {

    private final Calculator calculator = new Calculator();

    // This will result in a JUnitException!
    @TestFactory
    List<String> dynamicTestsWithInvalidReturnType() {
        return Arrays.asList("Hello");
    }

    @TestFactory
    Collection<DynamicTest> dynamicTestsFromCollection() {
        return Arrays.asList(
            dynamicTest("1st dynamic test", () -> assertTrue(isPalindrome("madam"))),
            dynamicTest("2nd dynamic test", () -> assertEquals(4, calculator.multiply(2, 2)))
        );
    }

    @TestFactory
    Iterable<DynamicTest> dynamicTestsFromIterable() {
        return Arrays.asList(
            dynamicTest("3rd dynamic test", () -> assertTrue(isPalindrome("madam"))),
            dynamicTest("4th dynamic test", () -> assertEquals(4, calculator.multiply(2, 2)))
        );
    }

    @TestFactory
    Iterator<DynamicTest> dynamicTestsFromIterator() {
        return Arrays.asList(
            dynamicTest("5th dynamic test", () -> assertTrue(isPalindrome("madam"))),
            dynamicTest("6th dynamic test", () -> assertEquals(4, calculator.multiply(2, 2)))
        ).iterator();
    }

    @TestFactory
    DynamicTest[] dynamicTestsFromArray() {
        return new DynamicTest[] {
            dynamicTest("7th dynamic test", () -> assertTrue(isPalindrome("madam"))),
            dynamicTest("8th dynamic test", () -> assertEquals(4, calculator.multiply(2, 2)))
        };
    }

    @TestFactory
    Stream<DynamicTest> dynamicTestsFromStream() {
        return Stream.of("racecar", "radar", "mom", "dad")
            .map(text -> dynamicTest(text, () -> assertTrue(isPalindrome(text))));
    }

    @TestFactory
    Stream<DynamicTest> dynamicTestsFromIntStream() {
        // Generates tests for the first 10 even integers.
        return IntStream.iterate(0, n -> n + 2).limit(10)
            .mapToObj(n -> dynamicTest("test" + n, () -> assertTrue(n % 2 == 0)));
    }

    @TestFactory
    Stream<DynamicTest> generateRandomNumberOfTestsFromIterator() {

        // Generates random positive integers between 0 and 100 until
        // a number evenly divisible by 7 is encountered.
        Iterator<Integer> inputGenerator = new Iterator<Integer>() {

            Random random = new Random();
            int current;

            @Override
            public boolean hasNext() {
                current = random.nextInt(100);
                return current % 7 != 0;
            }

            @Override
            public Integer next() {
                return current;
            }
        };

        // Generates display names like: input:5, input:37, input:85, etc.
        Function<Integer, String> displayNameGenerator = (input) -> "input:" + input;

        // Executes tests based on the current input value.
        ThrowingConsumer<Integer> testExecutor = (input) -> assertTrue(input % 7 != 0);

        // Returns a stream of dynamic tests.
        return DynamicTest.stream(inputGenerator, displayNameGenerator, testExecutor);
    }

    @TestFactory
    Stream<DynamicTest> dynamicTestsFromStreamFactoryMethod() {
        // Stream of palindromes to check
        Stream<String> inputStream = Stream.of("racecar", "radar", "mom", "dad");

        // Generates display names like: racecar is a palindrome
        Function<String, String> displayNameGenerator = text -> text + " is a palindrome";

        // Executes tests based on the current input value.
        ThrowingConsumer<String> testExecutor = text -> assertTrue(isPalindrome(text));

        // Returns a stream of dynamic tests.
        return DynamicTest.stream(inputStream, displayNameGenerator, testExecutor);
    }

    @TestFactory
    Stream<DynamicTest> dynamicTestsFromStreamFactoryMethodWithNames() {
        // Stream of palindromes to check
        Stream<Named<String>> inputStream = Stream.of(
                named("racecar is a palindrome", "racecar"),
                named("radar is also a palindrome", "radar"),
                named("mom also seems to be a palindrome", "mom"),
                named("dad is yet another palindrome", "dad")
            );

        // Returns a stream of dynamic tests.
        return DynamicTest.stream(inputStream,
            text -> assertTrue(isPalindrome(text)));
    }

    @TestFactory
    Stream<DynamicNode> dynamicTestsWithContainers() {
        return Stream.of("A", "B", "C")
            .map(input -> dynamicContainer("Container " + input, Stream.of(
                dynamicTest("not null", () -> assertNotNull(input)),
                dynamicContainer("properties", Stream.of(
                    dynamicTest("length > 0", () -> assertTrue(input.length() > 0)),
                    dynamicTest("not empty", () -> assertFalse(input.isEmpty()))
                ))
            )));
    }

    @TestFactory
    DynamicNode dynamicNodeSingleTest() {
        return dynamicTest("'pop' is a palindrome", () -> assertTrue(isPalindrome("pop")));
    }

    @TestFactory
    DynamicNode dynamicNodeSingleContainer() {
        return dynamicContainer("palindromes",
            Stream.of("racecar", "radar", "mom", "dad")
                .map(text -> dynamicTest(text, () -> assertTrue(isPalindrome(text)))
        ));
    }

}
```

#### [](#writing-tests-dynamic-tests-uri-test-source)2.18.2. URI Test Sources for Dynamic Tests ####

The JUnit Platform provides `TestSource`, a representation of the source of a test or
container used to navigate to its location by IDEs and build tools.

The `TestSource` for a dynamic test or dynamic container can be constructed from a`java.net.URI` which can be supplied via the `DynamicTest.dynamicTest(String, URI,
Executable)` or `DynamicContainer.dynamicContainer(String, URI, Stream)` factory method,
respectively. The `URI` will be converted to one of the following `TestSource`implementations.

`ClasspathResourceSource`

If the `URI` contains the `classpath` scheme — for example,`classpath:/test/foo.xml?line=20,column=2`.

`DirectorySource`

If the `URI` represents a directory present in the file system.

`FileSource`

If the `URI` represents a file present in the file system.

`MethodSource`

If the `URI` contains the `method` scheme and the fully qualified method name (FQMN) — for example, `method:org.junit.Foo#bar(java.lang.String, java.lang.String[])`. Please
refer to the Javadoc for `DiscoverySelectors.selectMethod(String)` for the supported
formats for a FQMN.

`ClassSource`

If the `URI` contains the `class` scheme and the fully qualified class name — for example, `class:org.junit.Foo?line=42`.

`UriSource`

If none of the above `TestSource` implementations are applicable.

### [](#writing-tests-declarative-timeouts)2.19. Timeouts ###

The `@Timeout` annotation allows one to declare that a test, test factory, test template,
or lifecycle method should fail if its execution time exceeds a given duration. The time
unit for the duration defaults to seconds but is configurable.

The following example shows how `@Timeout` is applied to lifecycle and test methods.

```
class TimeoutDemo {

    @BeforeEach
    @Timeout(5)
    void setUp() {
        // fails if execution time exceeds 5 seconds
    }

    @Test
    @Timeout(value = 500, unit = TimeUnit.MILLISECONDS)
    void failsIfExecutionTimeExceeds500Milliseconds() {
        // fails if execution time exceeds 500 milliseconds
    }

    @Test
    @Timeout(value = 500, unit = TimeUnit.MILLISECONDS, threadMode = ThreadMode.SEPARATE_THREAD)
    void failsIfExecutionTimeExceeds500MillisecondsInSeparateThread() {
        // fails if execution time exceeds 500 milliseconds, the test code is executed in a separate thread
    }

}
```

To apply the same timeout to all test methods within a test class and all of its `@Nested`classes, you can declare the `@Timeout` annotation at the class level. It will then be
applied to all test, test factory, and test template methods within that class and its`@Nested` classes unless overridden by a `@Timeout` annotation on a specific method or`@Nested` class. Please note that `@Timeout` annotations declared at the class level are
not applied to lifecycle methods.

Declaring `@Timeout` on a `@TestFactory` method checks that the factory method returns
within the specified duration but does not verify the execution time of each individual`DynamicTest` generated by the factory. Please use`assertTimeout()` or `assertTimeoutPreemptively()` for that purpose.

If `@Timeout` is present on a `@TestTemplate` method — for example, a `@RepeatedTest` or`@ParameterizedTest` — each invocation will have the given timeout applied to it.

#### [](#writing-tests-declarative-timeouts-thread-mode)2.19.1. Thread mode ####

The timeout can be applied using one of the following three thread modes: `SAME_THREAD`,`SEPARATE_THREAD`, or `INFERRED`.

When `SAME_THREAD` is used, the execution of the annotated method proceeds in the main
thread of the test. If the timeout is exceeded, the main thread is interrupted from
another thread. This is done to ensure interoperability with frameworks such as Spring
that make use of mechanisms that are sensitive to the currently running thread — for
example, `ThreadLocal` transaction management.

On the contrary when `SEPARATE_THREAD` is used, like the `assertTimeoutPreemptively()`assertion, the execution of the annotated method proceeds in a separate thread, this
can lead to undesirable side effects, see [Preemptive Timeouts with `assertTimeoutPreemptively()`](#writing-tests-assertions-preemptive-timeouts).

When `INFERRED` (default) thread mode is used, the thread mode is resolved via the`junit.jupiter.execution.timeout.thread.mode.default` configuration parameter. If the
provided configuration parameter is invalid or not present then `SAME_THREAD` is used as
fallback.

#### [](#writing-tests-declarative-timeouts-default-timeouts)2.19.2. Default Timeouts ####

The following [configuration parameters](#running-tests-config-params) can be used to
specify default timeouts for all methods of a certain category unless they or an enclosing
test class is annotated with `@Timeout`:

`junit.jupiter.execution.timeout.default`

Default timeout for all testable and lifecycle methods

`junit.jupiter.execution.timeout.testable.method.default`

Default timeout for all testable methods

`junit.jupiter.execution.timeout.test.method.default`

Default timeout for `@Test` methods

`junit.jupiter.execution.timeout.testtemplate.method.default`

Default timeout for `@TestTemplate` methods

`junit.jupiter.execution.timeout.testfactory.method.default`

Default timeout for `@TestFactory` methods

`junit.jupiter.execution.timeout.lifecycle.method.default`

Default timeout for all lifecycle methods

`junit.jupiter.execution.timeout.beforeall.method.default`

Default timeout for `@BeforeAll` methods

`junit.jupiter.execution.timeout.beforeeach.method.default`

Default timeout for `@BeforeEach` methods

`junit.jupiter.execution.timeout.aftereach.method.default`

Default timeout for `@AfterEach` methods

`junit.jupiter.execution.timeout.afterall.method.default`

Default timeout for `@AfterAll` methods

More specific configuration parameters override less specific ones. For example,`junit.jupiter.execution.timeout.test.method.default` overrides`junit.jupiter.execution.timeout.testable.method.default` which overrides`junit.jupiter.execution.timeout.default`.

The values of such configuration parameters must be in the following, case-insensitive
format: `<number> [ns|μs|ms|s|m|h|d]`. The space between the number and the unit may be
omitted. Specifying no unit is equivalent to using seconds.

|Parameter value|           Equivalent annotation           |
|---------------|-------------------------------------------|
|     `42`      |              `@Timeout(42)`               |
|    `42 ns`    |`@Timeout(value = 42, unit = NANOSECONDS)` |
|    `42 μs`    |`@Timeout(value = 42, unit = MICROSECONDS)`|
|    `42 ms`    |`@Timeout(value = 42, unit = MILLISECONDS)`|
|    `42 s`     |  `@Timeout(value = 42, unit = SECONDS)`   |
|    `42 m`     |  `@Timeout(value = 42, unit = MINUTES)`   |
|    `42 h`     |   `@Timeout(value = 42, unit = HOURS)`    |
|    `42 d`     |    `@Timeout(value = 42, unit = DAYS)`    |

#### [](#writing-tests-declarative-timeouts-polling)2.19.3. Using @Timeout for Polling Tests ####

When dealing with asynchronous code, it is common to write tests that poll while waiting
for something to happen before performing any assertions. In some cases you can rewrite
the logic to use a `CountDownLatch` or another synchronization mechanism, but sometimes
that is not possible — for example, if the subject under test sends a message to a channel
in an external message broker and assertions cannot be performed until the message has
been successfully sent through the channel. Asynchronous tests like these require some
form of timeout to ensure they don’t hang the test suite by executing indefinitely, as
would be the case if an asynchronous message never gets successfully delivered.

By configuring a timeout for an asynchronous test that polls, you can ensure that the test
does not execute indefinitely. The following example demonstrates how to achieve this with
JUnit Jupiter’s `@Timeout` annotation. This technique can be used to implement "poll
until" logic very easily.

```
@Test
@Timeout(5) // Poll at most 5 seconds
void pollUntil() throws InterruptedException {
    while (asynchronousResultNotAvailable()) {
        Thread.sleep(250); // custom poll interval
    }
    // Obtain the asynchronous result and perform assertions
}
```

|   |If you need more control over polling intervals and greater flexibility with<br/>asynchronous tests, consider using a dedicated library such as[Awaitility](https://github.com/awaitility/awaitility).|
|---|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

#### [](#writing-tests-declarative-timeouts-mode)2.19.4. Disable @Timeout Globally ####

When stepping through your code in a debug session, a fixed timeout limit may influence
the result of the test, e.g. mark the test as failed although all assertions were met.

JUnit Jupiter supports the `junit.jupiter.execution.timeout.mode` configuration parameter
to configure when timeouts are applied. There are three modes: `enabled`, `disabled`,
and `disabled_on_debug`. The default mode is `enabled`.
A VM runtime is considered to run in debug mode when one of its input parameters starts
with `-agentlib:jdwp` or `-Xrunjdwp`.
This heuristic is queried by the `disabled_on_debug` mode.

### [](#writing-tests-parallel-execution)2.20. Parallel Execution ###

By default, JUnit Jupiter tests are run sequentially in a single thread. Running tests in
parallel — for example, to speed up execution — is available as an opt-in feature since
version 5.3. To enable parallel execution, set the`junit.jupiter.execution.parallel.enabled` configuration parameter to `true` — for
example, in `junit-platform.properties` (see [Configuration Parameters](#running-tests-config-params) for other
options).

Please note that enabling this property is only the first step required to execute tests
in parallel. If enabled, test classes and methods will still be executed sequentially by
default. Whether or not a node in the test tree is executed concurrently is controlled by
its execution mode. The following two modes are available.

`SAME_THREAD`

Force execution in the same thread used by the parent. For example, when used on a test
method, the test method will be executed in the same thread as any `@BeforeAll` or`@AfterAll` methods of the containing test class.

`CONCURRENT`

Execute concurrently unless a resource lock forces execution in the same thread.

By default, nodes in the test tree use the `SAME_THREAD` execution mode. You can change
the default by setting the `junit.jupiter.execution.parallel.mode.default` configuration
parameter. Alternatively, you can use the `[@Execution](../api/org.junit.jupiter.api/org/junit/jupiter/api/parallel/Execution.html)` annotation to change the
execution mode for the annotated element and its subelements (if any) which allows you to
activate parallel execution for individual test classes, one by one.

Configuration parameters to execute all tests in parallel

```
junit.jupiter.execution.parallel.enabled = true
junit.jupiter.execution.parallel.mode.default = concurrent
```

The default execution mode is applied to all nodes of the test tree with a few notable
exceptions, namely test classes that use the `Lifecycle.PER_CLASS` mode or a`[MethodOrderer](../api/org.junit.jupiter.api/org/junit/jupiter/api/MethodOrderer.html)` (except for `[MethodOrderer.Random](../api/org.junit.jupiter.api/org/junit/jupiter/api/MethodOrderer.Random.html)`). In the former case, test authors
have to ensure that the test class is thread-safe; in the latter, concurrent execution
might conflict with the configured execution order. Thus, in both cases, test methods in
such test classes are only executed concurrently if the `@Execution(CONCURRENT)`annotation is present on the test class or method.

When parallel execution is enabled and a default `[ClassOrderer](../api/org.junit.jupiter.api/org/junit/jupiter/api/ClassOrderer.html)` is registered (see[Class Order](#writing-tests-test-execution-order-classes) for details), top-level test classes will
initially be sorted accordingly and scheduled in that order. However, they are not
guaranteed to be started in exactly that order since the threads they are executed on are
not controlled directly by JUnit.

All nodes of the test tree that are configured with the `CONCURRENT` execution mode will
be executed fully in parallel according to the provided[configuration](#writing-tests-parallel-execution-config) while observing the
declarative [synchronization](#writing-tests-parallel-execution-synchronization)mechanism. Please note that [Capturing Standard Output/Error](#running-tests-capturing-output) needs to be enabled
separately.

In addition, you can configure the default execution mode for top-level classes by setting
the `junit.jupiter.execution.parallel.mode.classes.default` configuration parameter. By
combining both configuration parameters, you can configure classes to run in parallel but
their methods in the same thread:

Configuration parameters to execute top-level classes in parallel but methods in same thread

```
junit.jupiter.execution.parallel.enabled = true
junit.jupiter.execution.parallel.mode.default = same_thread
junit.jupiter.execution.parallel.mode.classes.default = concurrent
```

The opposite combination will run all methods within one class in parallel, but top-level
classes will run sequentially:

Configuration parameters to execute top-level classes sequentially but their methods in parallel

```
junit.jupiter.execution.parallel.enabled = true
junit.jupiter.execution.parallel.mode.default = concurrent
junit.jupiter.execution.parallel.mode.classes.default = same_thread
```

The following diagram illustrates how the execution of two top-level test classes `A` and`B` with two test methods per class behaves for all four combinations of`junit.jupiter.execution.parallel.mode.default` and`junit.jupiter.execution.parallel.mode.classes.default` (see labels in first column).

![writing tests execution mode](images/writing-tests_execution_mode.svg)

Default execution mode configuration combinations

If the `junit.jupiter.execution.parallel.mode.classes.default` configuration parameter is
not explicitly set, the value for `junit.jupiter.execution.parallel.mode.default` will be
used instead.

#### [](#writing-tests-parallel-execution-config)2.20.1. Configuration ####

Properties such as the desired parallelism and the maximum pool size can be configured
using a `[ParallelExecutionConfigurationStrategy](../api/org.junit.platform.engine/org/junit/platform/engine/support/hierarchical/ParallelExecutionConfigurationStrategy.html)`. The JUnit Platform provides two
implementations out of the box: `dynamic` and `fixed`. Alternatively, you may implement a`custom` strategy.

To select a strategy, set the `junit.jupiter.execution.parallel.config.strategy`configuration parameter to one of the following options.

`dynamic`

Computes the desired parallelism based on the number of available processors/cores
multiplied by the `junit.jupiter.execution.parallel.config.dynamic.factor`configuration parameter (defaults to `1`).
The optional `junit.jupiter.execution.parallel.config.dynamic.max-pool-size-factor`configuration parameter can be used to limit the maximum number of threads.

`fixed`

Uses the mandatory `junit.jupiter.execution.parallel.config.fixed.parallelism`configuration parameter as the desired parallelism.
The optional `junit.jupiter.execution.parallel.config.fixed.max-pool-size`configuration parameter can be used to limit the maximum number of threads.

`custom`

Allows you to specify a custom `[ParallelExecutionConfigurationStrategy](../api/org.junit.platform.engine/org/junit/platform/engine/support/hierarchical/ParallelExecutionConfigurationStrategy.html)`implementation via the mandatory `junit.jupiter.execution.parallel.config.custom.class`configuration parameter to determine the desired configuration.

If no configuration strategy is set, JUnit Jupiter uses the `dynamic` configuration
strategy with a factor of `1`. Consequently, the desired parallelism will be equal to the
number of available processors/cores.

|   |Parallelism alone does not imply maximum number of concurrent threads<br/><br/>By default JUnit Jupiter does not guarantee that the number of concurrently<br/>executing tests will not exceed the configured parallelism. For example, when using one<br/>of the synchronization mechanisms described in the next section, the `ForkJoinPool` that<br/>is used behind the scenes may spawn additional threads to ensure execution continues with<br/>sufficient parallelism.<br/>If you require such guarantees, with Java 9+, it is possible to limit the maximum number<br/>of concurrent threads by controlling the maximum pool size of the `dynamic`, `fixed` and`custom` strategies.|
|---|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

##### [](#writing-tests-parallel-execution-config-properties)Relevant properties #####

The following table lists relevant properties for configuring parallel execution. See[Configuration Parameters](#running-tests-config-params) for details on how to set such properties.

|                                Property                                |                                                                                                         Description                                                                                                         |                                                Supported Values                                                |                                                          Default Value                                                           |
|------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------|
|              ``junit.jupiter.execution.parallel.enabled``              |                                                                                               Enable parallel test execution                                                                                                |                                          * `true`<br/><br/>* `false`                                           |                                                            ``false``                                                             |
|           ``junit.jupiter.execution.parallel.mode.default``            |                                                                                      Default execution mode of nodes in the test tree                                                                                       |                                    * `concurrent`<br/><br/>* `same_thread`                                     |                                                         ``same_thread``                                                          |
|       ``junit.jupiter.execution.parallel.mode.classes.default``        |                                                                                         Default execution mode of top-level classes                                                                                         |                                    * `concurrent`<br/><br/>* `same_thread`                                     |                                                         ``same_thread``                                                          |
|          ``junit.jupiter.execution.parallel.config.strategy``          |                                                                              Execution strategy for desired parallelism and maximum pool size                                                                               |                               * `dynamic`<br/><br/>* `fixed`<br/><br/>* `custom`                               |                                                           ``dynamic``                                                            |
|       ``junit.jupiter.execution.parallel.config.dynamic.factor``       |                                     Factor to be multiplied by the number of available processors/cores to determine the desired parallelism for the ``dynamic`` configuration strategy                                     |                                           a positive decimal number                                            |                                                             ``1.0``                                                              |
|``junit.jupiter.execution.parallel.config.dynamic.max-pool-size-factor``|Factor to be multiplied by the number of available processors/cores and the value of `junit.jupiter.execution.parallel.config.dynamic.factor` to determine the desired parallelism for the ``dynamic`` configuration strategy|                       a positive decimal number, must be greater than or equal to `1.0`                        |256 + the value of `junit.jupiter.execution.parallel.config.dynamic.factor` multiplied by the number of available processors/cores|
|      ``junit.jupiter.execution.parallel.config.dynamic.saturate``      |                                                             Disable saturation of the underlying fork-join pool for the ``dynamic`` configuration<br/>strategy                                                              |                                          * `true`<br/><br/>* `false`                                           |                                                             ``true``                                                             |
|     ``junit.jupiter.execution.parallel.config.fixed.parallelism``      |                                                                                Desired parallelism for the ``fixed`` configuration strategy                                                                                 |                                               a positive integer                                               |                                                         no default value                                                         |
|    ``junit.jupiter.execution.parallel.config.fixed.max-pool-size``     |                                                             Desired maximum pool size of the underlying fork-join pool for the ``fixed`` configuration strategy                                                             |a positive integer, must be greater than or equal to `junit.jupiter.execution.parallel.config.fixed.parallelism`|                          256 + the value of `junit.jupiter.execution.parallel.config.fixed.parallelism`                          |
|       ``junit.jupiter.execution.parallel.config.fixed.saturate``       |                                                                Disable saturation of the underlying fork-join pool for the ``fixed`` configuration strategy                                                                 |                                          * `true`<br/><br/>* `false`                                           |                                                             ``true``                                                             |
|        ``junit.jupiter.execution.parallel.config.custom.class``        |                                               Fully qualified class name of the *ParallelExecutionConfigurationStrategy* to be used for the ``custom`` configuration strategy                                               |                                   for example, *org.example.CustomStrategy*                                    |                                                         no default value                                                         |

#### [](#writing-tests-parallel-execution-synchronization)2.20.2. Synchronization ####

In addition to controlling the execution mode using the `[@Execution](../api/org.junit.jupiter.api/org/junit/jupiter/api/parallel/Execution.html)` annotation, JUnit
Jupiter provides another annotation-based declarative synchronization mechanism. The`[@ResourceLock](../api/org.junit.jupiter.api/org/junit/jupiter/api/parallel/ResourceLock.html)` annotation allows you to declare that a test class or method uses a
specific shared resource that requires synchronized access to ensure reliable test
execution. The shared resource is identified by a unique name which is a `String`. The
name can be user-defined or one of the predefined constants in `[Resources](../api/org.junit.jupiter.api/org/junit/jupiter/api/parallel/Resources.html)`:`SYSTEM_PROPERTIES`, `SYSTEM_OUT`, `SYSTEM_ERR`, `LOCALE`, or `TIME_ZONE`.

If the tests in the following example were run in parallel *without* the use of[@ResourceLock](../api/org.junit.jupiter.api/org/junit/jupiter/api/parallel/ResourceLock.html), they would be *flaky*. Sometimes they would pass, and at other times they
would fail due to the inherent race condition of writing and then reading the same JVM
System Property.

When access to shared resources is declared using the `[@ResourceLock](../api/org.junit.jupiter.api/org/junit/jupiter/api/parallel/ResourceLock.html)` annotation, the
JUnit Jupiter engine uses this information to ensure that no conflicting tests are run in
parallel.

|   |Running tests in isolation<br/><br/>If most of your test classes can be run in parallel without any synchronization but you<br/>have some test classes that need to run in isolation, you can mark the latter with the`[@Isolated](../api/org.junit.jupiter.api/org/junit/jupiter/api/parallel/Isolated.html)` annotation. Tests in such classes are executed sequentially without any other<br/>tests running at the same time.|
|---|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

In addition to the `String` that uniquely identifies the shared resource, you may specify
an access mode. Two tests that require `READ` access to a shared resource may run in
parallel with each other but not while any other test that requires `READ_WRITE` access
to the same shared resource is running.

```
@Execution(CONCURRENT)
class SharedResourcesDemo {

    private Properties backup;

    @BeforeEach
    void backup() {
        backup = new Properties();
        backup.putAll(System.getProperties());
    }

    @AfterEach
    void restore() {
        System.setProperties(backup);
    }

    @Test
    @ResourceLock(value = SYSTEM_PROPERTIES, mode = READ)
    void customPropertyIsNotSetByDefault() {
        assertNull(System.getProperty("my.prop"));
    }

    @Test
    @ResourceLock(value = SYSTEM_PROPERTIES, mode = READ_WRITE)
    void canSetCustomPropertyToApple() {
        System.setProperty("my.prop", "apple");
        assertEquals("apple", System.getProperty("my.prop"));
    }

    @Test
    @ResourceLock(value = SYSTEM_PROPERTIES, mode = READ_WRITE)
    void canSetCustomPropertyToBanana() {
        System.setProperty("my.prop", "banana");
        assertEquals("banana", System.getProperty("my.prop"));
    }

}
```

### [](#writing-tests-built-in-extensions)2.21. Built-in Extensions ###

While the JUnit team encourages reusable extensions to be packaged and maintained in
separate libraries, the JUnit Jupiter API artifact includes a few user-facing extension
implementations that are considered so generally useful that users shouldn’t have to add
another dependency.

#### [](#writing-tests-built-in-extensions-TempDirectory)2.21.1. The TempDirectory Extension ####

The built-in `[TempDirectory](https://github.com/junit-team/junit5/tree/r5.10.2/junit-jupiter-engine/src/main/java/org/junit/jupiter/engine/extension/TempDirectory.java)` extension is used to create and clean up a temporary
directory for an individual test or all tests in a test class. It is registered by
default. To use it, annotate a non-final, unassigned field of type `java.nio.file.Path` or`java.io.File` with `[@TempDir](../api/org.junit.jupiter.api/org/junit/jupiter/api/io/TempDir.html)` or add a parameter of type `java.nio.file.Path` or`java.io.File` annotated with `@TempDir` to a lifecycle method or test method.

For example, the following test declares a parameter annotated with `@TempDir` for a
single test method, creates and writes to a file in the temporary directory, and checks
its content.

A test method that requires a temporary directory

```
@Test
void writeItemsToFile(@TempDir Path tempDir) throws IOException {
    Path file = tempDir.resolve("test.txt");

    new ListWriter(file).write("a", "b", "c");

    assertEquals(singletonList("a,b,c"), Files.readAllLines(file));
}
```

You can inject multiple temporary directories by specifying multiple annotated parameters.

A test method that requires multiple temporary directories

```
@Test
void copyFileFromSourceToTarget(@TempDir Path source, @TempDir Path target) throws IOException {
    Path sourceFile = source.resolve("test.txt");
    new ListWriter(sourceFile).write("a", "b", "c");

    Path targetFile = Files.copy(sourceFile, target.resolve("test.txt"));

    assertNotEquals(sourceFile, targetFile);
    assertEquals(singletonList("a,b,c"), Files.readAllLines(targetFile));
}
```

|   |To revert to the old behavior of using a single temporary directory for the<br/>entire test class or method (depending on which level the annotation is used), you can set<br/>the `junit.jupiter.tempdir.scope` configuration parameter to `per_context`. However,<br/>please note that this option is deprecated and will be removed in a future release.|
|---|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

`@TempDir` is not supported on constructor parameters. If you wish to retain a single
reference to a temp directory across lifecycle methods and the current test method, please
use field injection by annotating an instance field with `@TempDir`.

The following example stores a *shared* temporary directory in a `static` field. This
allows the same `sharedTempDir` to be used in all lifecycle methods and test methods of
the test class. For better isolation, you should use an instance field so that each test
method uses a separate directory.

A test class that shares a temporary directory across test methods

```
class SharedTempDirectoryDemo {

    @TempDir
    static Path sharedTempDir;

    @Test
    void writeItemsToFile() throws IOException {
        Path file = sharedTempDir.resolve("test.txt");

        new ListWriter(file).write("a", "b", "c");

        assertEquals(singletonList("a,b,c"), Files.readAllLines(file));
    }

    @Test
    void anotherTestThatUsesTheSameTempDir() {
        // use sharedTempDir
    }

}
```

The `@TempDir` annotation has an optional `cleanup` attribute that can be set to either`NEVER`, `ON_SUCCESS`, or `ALWAYS`. If the cleanup mode is set to `NEVER`, temporary
directories are not deleted after a test completes. If it is set to `ON_SUCCESS`,
temporary directories are deleted only after a test completed successfully.

The default cleanup mode is `ALWAYS`. You can use the`junit.jupiter.tempdir.cleanup.mode.default`[configuration parameter](#running-tests-config-params) to override this default.

A test class with a temporary directory that doesn’t get cleaned up

```
class CleanupModeDemo {

    @Test
    void fileTest(@TempDir(cleanup = ON_SUCCESS) Path tempDir) {
        // perform test
    }

}
```

`@TempDir` supports the programmatic creation of temporary directories via the optional`factory` attribute. This is typically used to gain control over the temporary directory
creation, like defining the parent directory or the file system that should be used.

Factories can be created by implementing `TempDirFactory`. Implementations must provide a
no-args constructor and should not make any assumptions regarding when and how many times
they are instantiated, but they can assume that their `createTempDirectory(…​)` and`close()` methods will both be called once per instance, in this order, and from the same
thread.

The default implementation available in Jupiter delegates the directory creation to`java.nio.file.Files::createTempDirectory`, passing `junit` as the prefix string to be
used in generating the directory’s name.

The following example defines a factory that uses the test name as the directory name
prefix instead of the `junit` constant value.

A test class with a temporary directory having the test name as the directory name prefix

```
class TempDirFactoryDemo {

    @Test
    void factoryTest(@TempDir(factory = Factory.class) Path tempDir) {
        assertTrue(tempDir.getFileName().toString().startsWith("factoryTest"));
    }

    static class Factory implements TempDirFactory {

        @Override
        public Path createTempDirectory(AnnotatedElementContext elementContext, ExtensionContext extensionContext)
                throws IOException {
            return Files.createTempDirectory(extensionContext.getRequiredTestMethod().getName());
        }

    }

}
```

It’s also possible to use an in-memory file system like `[Jimfs](https://google.github.io/jimfs/)` for the creation of the
temporary directory. The following example demonstrates how to achieve that.

A test class with a temporary directory created with the Jimfs in-memory file system

```
class InMemoryTempDirDemo {

    @Test
    void test(@TempDir(factory = JimfsTempDirFactory.class) Path tempDir) {
        // perform test
    }

    static class JimfsTempDirFactory implements TempDirFactory {

        private final FileSystem fileSystem = Jimfs.newFileSystem(Configuration.unix());

        @Override
        public Path createTempDirectory(AnnotatedElementContext elementContext, ExtensionContext extensionContext)
                throws IOException {
            return Files.createTempDirectory(fileSystem.getPath("/"), "junit");
        }

        @Override
        public void close() throws IOException {
            fileSystem.close();
        }

    }

}
```

`@TempDir` can also be used as a [meta-annotation](#writing-tests-meta-annotations) to
reduce repetition. The following code listing shows how to create a custom `@JimfsTempDir`annotation that can be used as a drop-in replacement for`@TempDir(factory = JimfsTempDirFactory.class)`.

A custom annotation meta-annotated with `@TempDir`

```
@Target({ ElementType.ANNOTATION_TYPE, ElementType.FIELD, ElementType.PARAMETER })
@Retention(RetentionPolicy.RUNTIME)
@TempDir(factory = JimfsTempDirFactory.class)
@interface JimfsTempDir {
}
```

The following example demonstrates how to use the custom `@JimfsTempDir` annotation.

A test class using the custom annotation

```
class JimfsTempDirAnnotationDemo {

    @Test
    void test(@JimfsTempDir Path tempDir) {
        // perform test
    }

}
```

Meta-annotations or additional annotations on the field or parameter the `TempDir`annotation is declared on might expose additional attributes to configure the factory.
Such annotations and related attributes can be accessed via the `AnnotatedElementContext`parameter of `createTempDirectory`.

You can use the `junit.jupiter.tempdir.factory.default`[configuration parameter](#running-tests-config-params) to specify the fully qualified
class name of the `TempDirFactory` you would like to use by default. Just like for
factories configured via the `factory` attribute of the `@TempDir` annotation,
the supplied class has to implement the `TempDirFactory` interface. The default factory
will be used for all `@TempDir` annotations unless the `factory` attribute of the
annotation specifies a different factory.

In summary, the factory for a temporary directory is determined according to the
following precedence rules:

1. The `factory` attribute of the `@TempDir` annotation, if present

2. The default `TempDirFactory` configured via the configuration
   parameter, if present

3. Otherwise, `org.junit.jupiter.api.io.TempDirFactory$Standard` will be used.

[](#migrating-from-junit4)3. Migrating from JUnit 4
----------

Although the JUnit Jupiter programming model and extension model do not support JUnit 4
features such as `Rules` and `Runners` natively, it is not expected that source code
maintainers will need to update all of their existing tests, test extensions, and custom
build test infrastructure to migrate to JUnit Jupiter.

Instead, JUnit provides a gentle migration path via a *JUnit Vintage test engine* which
allows existing tests based on JUnit 3 and JUnit 4 to be executed using the JUnit Platform
infrastructure. Since all classes and annotations specific to JUnit Jupiter reside under
the `org.junit.jupiter` base package, having both JUnit 4 and JUnit Jupiter in the
classpath does not lead to any conflicts. It is therefore safe to maintain existing JUnit
4 tests alongside JUnit Jupiter tests. Furthermore, since the JUnit team will continue to
provide maintenance and bug fix releases for the JUnit 4.x baseline, developers have
plenty of time to migrate to JUnit Jupiter on their own schedule.

### [](#migrating-from-junit4-running)3.1. Running JUnit 4 Tests on the JUnit Platform ###

Make sure that the `junit-vintage-engine` artifact is in your test runtime path. In that
case JUnit 3 and JUnit 4 tests will automatically be picked up by the JUnit Platform
launcher.

See the example projects in the [`junit5-samples`](https://github.com/junit-team/junit5-samples) repository to
find out how this is done with Gradle and Maven.

#### [](#migrating-from-junit4-categories-support)3.1.1. Categories Support ####

For test classes or methods that are annotated with `@Category`, the *JUnit Vintage test
engine* exposes the category’s fully qualified class name as a [tag](#running-tests-tags)for the corresponding test class or test method. For example, if a test method is
annotated with `@Category(Example.class)`, it will be tagged with `"com.acme.Example"`.
Similar to the `Categories` runner in JUnit 4, this information can be used to filter the
discovered tests before executing them (see [Running Tests](#running-tests) for details).

### [](#migrating-from-junit4-tips)3.2. Migration Tips ###

The following are topics that you should be aware of when migrating existing JUnit 4
tests to JUnit Jupiter.

* Annotations reside in the `org.junit.jupiter.api` package.

* Assertions reside in `org.junit.jupiter.api.Assertions`.

  * Note that you may continue to use assertion methods from `org.junit.Assert` or any
    other assertion library such as [AssertJ](https://assertj.github.io/doc/), [Hamcrest](https://hamcrest.org/JavaHamcrest/), [Truth](https://truth.dev/), etc.

* Assumptions reside in `org.junit.jupiter.api.Assumptions`.

  * Note that JUnit Jupiter 5.4 and later versions support methods from JUnit 4’s`org.junit.Assume` class for assumptions. Specifically, JUnit Jupiter supports JUnit
    4’s `AssumptionViolatedException` to signal that a test should be aborted instead of
    marked as a failure.

* `@Before` and `@After` no longer exist; use `@BeforeEach` and `@AfterEach` instead.

* `@BeforeClass` and `@AfterClass` no longer exist; use `@BeforeAll` and `@AfterAll`instead.

* `@Ignore` no longer exists: use `@Disabled` or one of the other built-in[execution conditions](#writing-tests-conditional-execution) instead

  * See also [JUnit 4 @Ignore Support](#migrating-from-junit4-ignore-annotation-support).

* `@Category` no longer exists; use `@Tag` instead.

* `@RunWith` no longer exists; superseded by `@ExtendWith`.

* `@Rule` and `@ClassRule` no longer exist; superseded by `@ExtendWith` and`@RegisterExtension`.

  * See also [Limited JUnit 4 Rule Support](#migrating-from-junit4-rule-support).

* `@Test(expected = …​)` and the `ExpectedException` rule no longer exist; use`Assertions.assertThrows(…​)` instead.

  * See [Limited JUnit 4 Rule Support](#migrating-from-junit4-rule-support) if you still need to use`ExpectedException`.

* Assertions and assumptions in JUnit Jupiter accept the failure message as their last
  argument instead of the first one.

  * See [Failure Message Arguments](#migrating-from-junit4-failure-message-arguments) for details.

### [](#migrating-from-junit4-rule-support)3.3. Limited JUnit 4 Rule Support ###

As stated above, JUnit Jupiter does not and will not support JUnit 4 rules natively. The
JUnit team realizes, however, that many organizations, especially large ones, are likely
to have large JUnit 4 code bases that make use of custom rules. To serve these
organizations and enable a gradual migration path the JUnit team has decided to support a
selection of JUnit 4 rules verbatim within JUnit Jupiter. This support is based on
adapters and is limited to those rules that are semantically compatible to the JUnit
Jupiter extension model, i.e. those that do not completely change the overall execution
flow of the test.

The `junit-jupiter-migrationsupport` module from JUnit Jupiter currently supports the
following three `Rule` types including subclasses of these types:

* `org.junit.rules.ExternalResource` (including `org.junit.rules.TemporaryFolder`)

* `org.junit.rules.Verifier` (including `org.junit.rules.ErrorCollector`)

* `org.junit.rules.ExpectedException`

As in JUnit 4, Rule-annotated fields as well as methods are supported. By using these
class-level extensions on a test class such `Rule` implementations in legacy code bases
can be *left unchanged* including the JUnit 4 rule import statements.

This limited form of `Rule` support can be switched on by the class-level annotation`[@EnableRuleMigrationSupport](../api/org.junit.jupiter.migrationsupport/org/junit/jupiter/migrationsupport/rules/EnableRuleMigrationSupport.html)`. This annotation is a *composed annotation* which enables
all rule migration support extensions: `VerifierSupport`, `ExternalResourceSupport`, and`ExpectedExceptionSupport`. You may alternatively choose to annotate your test class with`@EnableJUnit4MigrationSupport` which registers migration support for rules *and* JUnit
4’s `@Ignore` annotation (see [JUnit 4 @Ignore Support](#migrating-from-junit4-ignore-annotation-support)).

However, if you intend to develop a new extension for JUnit Jupiter please use the new
extension model of JUnit Jupiter instead of the rule-based model of JUnit 4.

### [](#migrating-from-junit4-ignore-annotation-support)3.4. JUnit 4 @Ignore Support ###

In order to provide a smooth migration path from JUnit 4 to JUnit Jupiter, the`junit-jupiter-migrationsupport` module provides support for JUnit 4’s `@Ignore`annotation analogous to Jupiter’s `[@Disabled](../api/org.junit.jupiter.api/org/junit/jupiter/api/Disabled.html)` annotation.

To use `@Ignore` with JUnit Jupiter based tests, configure a *test* dependency on the`junit-jupiter-migrationsupport` module in your build and then annotate your test class
with `@ExtendWith(IgnoreCondition.class)` or `[@EnableJUnit4MigrationSupport](../api/org.junit.jupiter.migrationsupport/org/junit/jupiter/migrationsupport/EnableJUnit4MigrationSupport.html)` (which
automatically registers the `IgnoreCondition` along with[Limited JUnit 4 Rule Support](#migrating-from-junit4-rule-support)). The `IgnoreCondition` is an`[ExecutionCondition](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/ExecutionCondition.html)` that disables test classes or test methods that are annotated with`@Ignore`.

```
import org.junit.Ignore;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.migrationsupport.EnableJUnit4MigrationSupport;

// @ExtendWith(IgnoreCondition.class)
@EnableJUnit4MigrationSupport
class IgnoredTestsDemo {

    @Ignore
    @Test
    void testWillBeIgnored() {
    }

    @Test
    void testWillBeExecuted() {
    }
}
```

### [](#migrating-from-junit4-failure-message-arguments)3.5. Failure Message Arguments ###

The `Assumptions` and `Assertions` classes in JUnit Jupiter declare arguments in a
different order than in JUnit 4. In JUnit 4 assertion and assumption methods accept the
failure message as the first argument; whereas, in JUnit Jupiter assertion and assumption
methods accept the failure message as the last argument.

For instance, the method `assertEquals` in JUnit 4 is declared as `assertEquals(String
message, Object expected, Object actual)`, but in JUnit Jupiter it is declared as`assertEquals(Object expected, Object actual, String message)`. The rationale for this is
that a failure message is *optional*, and optional arguments should be declared after
required arguments in a method signature.

The methods affected by this change are the following:

* Assertions

  * `assertTrue`

  * `assertFalse`

  * `assertNull`

  * `assertNotNull`

  * `assertEquals`

  * `assertNotEquals`

  * `assertArrayEquals`

  * `assertSame`

  * `assertNotSame`

  * `assertThrows`

* Assumptions

  * `assumeTrue`

  * `assumeFalse`

[](#running-tests)4. Running Tests
----------

### [](#running-tests-ide)4.1. IDE Support ###

#### [](#running-tests-ide-intellij-idea)4.1.1. IntelliJ IDEA ####

IntelliJ IDEA supports running tests on the JUnit Platform since version 2016.2. For
details please see the[post on the
IntelliJ IDEA blog](https://blog.jetbrains.com/idea/2016/08/using-junit-5-in-intellij-idea/). Note, however, that it is recommended to use IDEA 2017.3 or newer
since these newer versions of IDEA will download the following JARs automatically based
on the API version used in the project: `junit-platform-launcher`,`junit-jupiter-engine`, and `junit-vintage-engine`.

|   |IntelliJ IDEA releases prior to IDEA 2017.3 bundle specific versions of JUnit 5.<br/>Thus, if you want to use a newer version of JUnit Jupiter, execution of tests within the<br/>IDE might fail due to version conflicts. In such cases, please follow the instructions<br/>below to use a newer version of JUnit 5 than the one bundled with IntelliJ IDEA.|
|---|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

In order to use a different JUnit 5 version (e.g., 5.10.2), you may need to
include the corresponding versions of the `junit-platform-launcher`,`junit-jupiter-engine`, and `junit-vintage-engine` JARs in the classpath.

Additional Gradle Dependencies

```
testImplementation(platform("org.junit:junit-bom:5.10.2"))
testRuntimeOnly("org.junit.platform:junit-platform-launcher") {
  because("Only needed to run tests in a version of IntelliJ IDEA that bundles older versions")
}
testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine")
testRuntimeOnly("org.junit.vintage:junit-vintage-engine")
```

Additional Maven Dependencies

```
<!-- ... -->
<dependencies>
    <!-- Only needed to run tests in a version of IntelliJ IDEA that bundles older versions -->
    <dependency>
        <groupId>org.junit.platform</groupId>
        <artifactId>junit-platform-launcher</artifactId>
        <scope>test</scope>
    </dependency>
    <dependency>
        <groupId>org.junit.jupiter</groupId>
        <artifactId>junit-jupiter-engine</artifactId>
        <scope>test</scope>
    </dependency>
    <dependency>
        <groupId>org.junit.vintage</groupId>
        <artifactId>junit-vintage-engine</artifactId>
        <scope>test</scope>
    </dependency>
</dependencies>
<dependencyManagement>
    <dependencies>
        <dependency>
            <groupId>org.junit</groupId>
            <artifactId>junit-bom</artifactId>
            <version>5.10.2</version>
            <type>pom</type>
            <scope>import</scope>
        </dependency>
    </dependencies>
</dependencyManagement>
```

#### [](#running-tests-ide-eclipse)4.1.2. Eclipse ####

Eclipse IDE offers support for the JUnit Platform since the Eclipse Oxygen.1a (4.7.1a)
release.

For more information on using JUnit 5 in Eclipse consult the official *Eclipse support
for JUnit 5* section of the[Eclipse Project Oxygen.1a
(4.7.1a) - New and Noteworthy](https://www.eclipse.org/eclipse/news/4.7.1a/#junit-5-support) documentation.

#### [](#running-tests-ide-netbeans)4.1.3. NetBeans ####

NetBeans offers support for JUnit Jupiter and the JUnit Platform since the[Apache NetBeans 10.0 release](https://netbeans.apache.org/download/nb100/nb100.html).

For more information consult the JUnit 5 section of the[Apache NetBeans 10.0
release notes](https://netbeans.apache.org/download/nb100/index.html#_junit_5).

#### [](#running-tests-ide-vscode)4.1.4. Visual Studio Code ####

[Visual Studio Code](https://code.visualstudio.com/) supports JUnit Jupiter and the JUnit
Platform via the[Java Test
Runner](https://marketplace.visualstudio.com/items?itemName=vscjava.vscode-java-test) extension which is installed by default as part of the[Java
Extension Pack](https://marketplace.visualstudio.com/items?itemName=vscjava.vscode-java-pack).

For more information consult the *Testing* section of the[Java in Visual Studio Code](https://code.visualstudio.com/docs/languages/java#_testing)documentation.

#### [](#running-tests-ide-other)4.1.5. Other IDEs ####

If you are using an editor or IDE other than one of those listed in the previous sections,
the JUnit team provides two alternative solutions to assist you in using JUnit 5. You can
use the [Console Launcher](#running-tests-console-launcher) manually — for example, from the command line — or execute tests with a [JUnit 4 based Runner](#running-tests-junit-platform-runner) if
your IDE has built-in support for JUnit 4.

### [](#running-tests-build)4.2. Build Support ###

#### [](#running-tests-build-gradle)4.2.1. Gradle ####

Starting with [version 4.6](https://docs.gradle.org/4.6/release-notes.html), Gradle provides[native support](https://docs.gradle.org/current/userguide/java_testing.html#using_junit5)for executing tests on the JUnit Platform. To enable it, you need to specify`useJUnitPlatform()` within a `test` task declaration in `build.gradle`:

```
test {
    useJUnitPlatform()
}
```

Filtering by [tags](#running-tests-tags),[tag expressions](#running-tests-tag-expressions), or engines is also supported:

```
test {
    useJUnitPlatform {
        includeTags("fast", "smoke & feature-a")
        // excludeTags("slow", "ci")
        includeEngines("junit-jupiter")
        // excludeEngines("junit-vintage")
    }
}
```

Please refer to the[official Gradle documentation](https://docs.gradle.org/current/userguide/java_plugin.html#sec:java_test)for a comprehensive list of options.

##### [](#running-tests-build-gradle-bom)Aligning dependency versions #####

Unless you’re using Spring Boot which defines its own way of managing dependencies, it is
recommended to use the JUnit Platform BOM to align the versions of all JUnit 5 artifacts.

```
dependencies {
    testImplementation(platform("org.junit:junit-bom:5.10.2"))
}
```

Using the BOM allows you to omit the version when declaring dependencies on all artifacts
with the `org.junit.platform`, `org.junit.jupiter`, and `org.junit.vintage` group IDs.

|   |See [Spring Boot](#running-tests-build-spring-boot) for details on how to override the version<br/>of JUnit used in your Spring Boot application.|
|---|-------------------------------------------------------------------------------------------------------------------------------------------------|

##### [](#running-tests-build-gradle-config-params)Configuration Parameters #####

The standard Gradle `test` task currently does not provide a dedicated DSL to set JUnit
Platform [configuration parameters](#running-tests-config-params) to influence test
discovery and execution. However, you can provide configuration parameters within the
build script via system properties (as shown below) or via the`junit-platform.properties` file.

```
test {
    // ...
    systemProperty("junit.jupiter.conditions.deactivate", "*")
    systemProperty("junit.jupiter.extensions.autodetection.enabled", true)
    systemProperty("junit.jupiter.testinstance.lifecycle.default", "per_class")
    // ...
}
```

##### [](#running-tests-build-gradle-engines-configure)Configuring Test Engines #####

In order to run any tests at all, a `TestEngine` implementation must be on the classpath.

To configure support for JUnit Jupiter based tests, configure a `testImplementation` dependency
on the dependency-aggregating JUnit Jupiter artifact similar to the following.

```
dependencies {
    testImplementation("org.junit.jupiter:junit-jupiter:5.10.2") // version can be omitted when using the BOM
}
```

The JUnit Platform can run JUnit 4 based tests as long as you configure a `testImplementation`dependency on JUnit 4 and a `testRuntimeOnly` dependency on the JUnit Vintage `TestEngine`implementation similar to the following.

```
dependencies {
    testImplementation("junit:junit:4.13.2")
    testRuntimeOnly("org.junit.vintage:junit-vintage-engine:5.10.2") // version can be omitted when using the BOM
}
```

##### [](#running-tests-build-gradle-logging)Configuring Logging (optional) #####

JUnit uses the Java Logging APIs in the `java.util.logging` package (a.k.a. *JUL*) to
emit warnings and debug information. Please refer to the official documentation of`[LogManager](https://docs.oracle.com/javase/8/docs/api/java/util/logging/LogManager.html)` for configuration options.

Alternatively, it’s possible to redirect log messages to other logging frameworks such as[Log4j](https://logging.apache.org/log4j/2.x/) or [Logback](https://logback.qos.ch/). To use a logging framework that provides a custom implementation of`[LogManager](https://docs.oracle.com/javase/8/docs/api/java/util/logging/LogManager.html)`, set the `java.util.logging.manager` system property to the *fully
qualified class name* of the `[LogManager](https://docs.oracle.com/javase/8/docs/api/java/util/logging/LogManager.html)` implementation to use. The example below
demonstrates how to configure Log4j 2.x (see [Log4j JDK Logging Adapter](https://logging.apache.org/log4j/2.x/log4j-jul/index.html) for
details).

```
test {
    systemProperty("java.util.logging.manager", "org.apache.logging.log4j.jul.LogManager")
}
```

Other logging frameworks provide different means to redirect messages logged using`java.util.logging`. For example, for [Logback](https://logback.qos.ch/) you can use the[JUL to SLF4J Bridge](https://www.slf4j.org/legacy.html#jul-to-slf4j) by adding an
additional dependency to the runtime classpath.

#### [](#running-tests-build-maven)4.2.2. Maven ####

Starting with [version 2.22.0](https://issues.apache.org/jira/browse/SUREFIRE-1330), Maven
Surefire and Maven Failsafe provide[native support](https://maven.apache.org/surefire/maven-surefire-plugin/examples/junit-platform.html)for executing tests on the JUnit Platform. The `pom.xml` file in the`[junit5-jupiter-starter-maven](https://github.com/junit-team/junit5-samples/tree/r5.10.2/junit5-jupiter-starter-maven)` project demonstrates how to use the Maven Surefire plugin
and can serve as a starting point for configuring your Maven build.

|   |Use Maven Surefire/Failsafe 3.0.0-M4 or later to avoid interoperability issues<br/><br/>Maven Surefire/Failsafe 3.0.0-M4[introduced support](https://issues.apache.org/jira/browse/SUREFIRE-1585) for aligning the<br/>version of the JUnit Platform Launcher it uses with the JUnit Platform version found on<br/>the test runtime classpath. Therefore, it is recommended to use version 3.0.0-M4 or later<br/>to avoid interoperability issues.<br/><br/>Alternatively, you can add a test dependency on the matching version of the JUnit Platform<br/>Launcher to your Maven build as follows.<br/><br/>```<br/><dependency><br/>    <groupId>org.junit.platform</groupId><br/>    <artifactId>junit-platform-launcher</artifactId><br/>    <version>1.10.2</version><br/>    <scope>test</scope><br/></dependency><br/>```|
|---|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

##### [](#running-tests-build-maven-bom)Aligning dependency versions #####

Unless you’re using Spring Boot which defines its own way of managing dependencies, it is
recommended to use the JUnit Platform BOM to align the versions of all JUnit 5 artifacts.

```
<dependencyManagement>
    <dependencies>
        <dependency>
            <groupId>org.junit</groupId>
            <artifactId>junit-bom</artifactId>
            <version>5.10.2</version>
            <type>pom</type>
            <scope>import</scope>
        </dependency>
    </dependencies>
</dependencyManagement>
```

Using the BOM allows you to omit the version when declaring dependencies on all artifacts
with the `org.junit.platform`, `org.junit.jupiter`, and `org.junit.vintage` group IDs.

|   |See [Spring Boot](#running-tests-build-spring-boot) for details on how to override the version<br/>of JUnit used in your Spring Boot application.|
|---|-------------------------------------------------------------------------------------------------------------------------------------------------|

##### [](#running-tests-build-maven-engines-configure)Configuring Test Engines #####

In order to have Maven Surefire or Maven Failsafe run any tests at all, at least one`TestEngine` implementation must be added to the test classpath.

To configure support for JUnit Jupiter based tests, configure `test` scoped dependencies
on the JUnit Jupiter API and the JUnit Jupiter `TestEngine` implementation similar to the
following.

```
<!-- ... -->
<dependencies>
    <!-- ... -->
    <dependency>
        <groupId>org.junit.jupiter</groupId>
        <artifactId>junit-jupiter</artifactId>
        <version>5.10.2</version> <!-- can be omitted when using the BOM -->
        <scope>test</scope>
    </dependency>
    <!-- ... -->
</dependencies>
<build>
    <plugins>
        <plugin>
            <artifactId>maven-surefire-plugin</artifactId>
            <version>3.1.2</version>
        </plugin>
        <plugin>
            <artifactId>maven-failsafe-plugin</artifactId>
            <version>3.1.2</version>
        </plugin>
    </plugins>
</build>
<!-- ... -->
```

Maven Surefire and Maven Failsafe can run JUnit 4 based tests alongside Jupiter tests as
long as you configure `test` scoped dependencies on JUnit 4 and the JUnit Vintage`TestEngine` implementation similar to the following.

```
<!-- ... -->
<dependencies>
    <!-- ... -->
    <dependency>
        <groupId>junit</groupId>
        <artifactId>junit</artifactId>
        <version>4.13.2</version>
        <scope>test</scope>
    </dependency>
    <dependency>
        <groupId>org.junit.vintage</groupId>
        <artifactId>junit-vintage-engine</artifactId>
        <version>5.10.2</version> <!-- can be omitted when using the BOM -->
        <scope>test</scope>
    </dependency>
    <!-- ... -->
</dependencies>
<!-- ... -->
<build>
    <plugins>
        <plugin>
            <artifactId>maven-surefire-plugin</artifactId>
            <version>3.1.2</version>
        </plugin>
        <plugin>
            <artifactId>maven-failsafe-plugin</artifactId>
            <version>3.1.2</version>
        </plugin>
    </plugins>
</build>
<!-- ... -->
```

##### [](#running-tests-build-maven-filter-test-class-names)Filtering by Test Class Names #####

The Maven Surefire Plugin will scan for test classes whose fully qualified names match
the following patterns.

* `**/Test*.java`

* `**/*Test.java`

* `**/*Tests.java`

* `**/*TestCase.java`

Moreover, it will exclude all nested classes (including static member classes) by default.

Note, however, that you can override this default behavior by configuring explicit`include` and `exclude` rules in your `pom.xml` file. For example, to keep Maven Surefire
from excluding static member classes, you can override its exclude rules as follows.

Overriding exclude rules of Maven Surefire

```
<!-- ... -->
<build>
    <plugins>
        <plugin>
            <artifactId>maven-surefire-plugin</artifactId>
            <version>3.1.2</version>
            <configuration>
                <excludes>
                    <exclude/>
                </excludes>
            </configuration>
        </plugin>
    </plugins>
</build>
<!-- ... -->
```

Please see the[Inclusions and Exclusions of Tests](https://maven.apache.org/surefire/maven-surefire-plugin/examples/inclusion-exclusion.html)documentation for Maven Surefire for details.

##### [](#running-tests-build-maven-filter-tags)Filtering by Tags #####

You can filter tests by [tags](#running-tests-tags) or[tag expressions](#running-tests-tag-expressions) using the following configuration
properties.

* to include *tags* or *tag expressions*, use `groups`.

* to exclude *tags* or *tag expressions*, use `excludedGroups`.

```
<!-- ... -->
<build>
    <plugins>
        <plugin>
            <artifactId>maven-surefire-plugin</artifactId>
            <version>3.1.2</version>
            <configuration>
                <groups>acceptance | !feature-a</groups>
                <excludedGroups>integration, regression</excludedGroups>
            </configuration>
        </plugin>
    </plugins>
</build>
<!-- ... -->
```

##### [](#running-tests-build-maven-config-params)Configuration Parameters #####

You can set JUnit Platform [configuration parameters](#running-tests-config-params) to
influence test discovery and execution by declaring the `configurationParameters`property and providing key-value pairs using the Java `Properties` file syntax (as shown
below) or via the `junit-platform.properties` file.

```
<!-- ... -->
<build>
    <plugins>
        <plugin>
            <artifactId>maven-surefire-plugin</artifactId>
            <version>3.1.2</version>
            <configuration>
                <properties>
                    <configurationParameters>
                        junit.jupiter.conditions.deactivate = *
                        junit.jupiter.extensions.autodetection.enabled = true
                        junit.jupiter.testinstance.lifecycle.default = per_class
                    </configurationParameters>
                </properties>
            </configuration>
        </plugin>
    </plugins>
</build>
<!-- ... -->
```

#### [](#running-tests-build-ant)4.2.3. Ant ####

Starting with version `1.10.3`, [Ant](https://ant.apache.org/) has a[`junitlauncher`](https://ant.apache.org/manual/Tasks/junitlauncher.html) task that
provides native support for launching tests on the JUnit Platform. The `junitlauncher`task is solely responsible for launching the JUnit Platform and passing it the selected
collection of tests. The JUnit Platform then delegates to registered test engines to
discover and execute the tests.

The `junitlauncher` task attempts to align as closely as possible with native Ant
constructs such as[resource collections](https://ant.apache.org/manual/Types/resources.html#collection)for allowing users to select the tests that they want executed by test engines. This gives
the task a consistent and natural feel when compared to many other core Ant tasks.

Starting with version `1.10.6` of Ant, the `junitlauncher` task supports[forking the tests in a separate JVM](https://ant.apache.org/manual/Tasks/junitlauncher.html#fork).

The `build.xml` file in the `[junit5-jupiter-starter-ant](https://github.com/junit-team/junit5-samples/tree/r5.10.2/junit5-jupiter-starter-ant)` project demonstrates how to use
the task and can serve as a starting point.

##### [](#basic-usage)Basic Usage #####

The following example demonstrates how to configure the `junitlauncher` task to select a
single test class (i.e., `org.myapp.test.MyFirstJUnit5Test`).

```
<path id="test.classpath">
    <!-- The location where you have your compiled classes -->
    <pathelement location="${build.classes.dir}" />
</path>

<!-- ... -->

<junitlauncher>
    <classpath refid="test.classpath" />
    <test name="org.myapp.test.MyFirstJUnit5Test" />
</junitlauncher>
```

The `test` element allows you to specify a single test class that you want to be selected
and executed. The `classpath` element allows you to specify the classpath to be used to
launch the JUnit Platform. This classpath will also be used to locate test classes that
are part of the execution.

The following example demonstrates how to configure the `junitlauncher` task to select
test classes from multiple locations.

```
<path id="test.classpath">
    <!-- The location where you have your compiled classes -->
    <pathelement location="${build.classes.dir}" />
</path>
<!-- ... -->
<junitlauncher>
    <classpath refid="test.classpath" />
    <testclasses outputdir="${output.dir}">
        <fileset dir="${build.classes.dir}">
            <include name="org/example/**/demo/**/" />
        </fileset>
        <fileset dir="${some.other.dir}">
            <include name="org/myapp/**/" />
        </fileset>
    </testclasses>
</junitlauncher>
```

In the above example, the `testclasses` element allows you to select multiple test
classes that reside in different locations.

For further details on usage and configuration options please refer to the official Ant
documentation for the[`junitlauncher` task](https://ant.apache.org/manual/Tasks/junitlauncher.html).

#### [](#running-tests-build-spring-boot)4.2.4. Spring Boot ####

[Spring Boot](https://spring.io/projects/spring-boot) provides automatic support for
managing the version of JUnit used in your project. In addition, the`spring-boot-starter-test` artifact automatically includes testing libraries such as JUnit
Jupiter, AssertJ, Mockito, etc.

If your build relies on dependency management support from Spring Boot, you should not
import the [`junit-bom`](#dependency-metadata-junit-bom) in your build script since that
will result in duplicate (and potentially conflicting) management of JUnit dependencies.

If you need to override the version of a dependency used in your Spring Boot application,
you have to override the exact name of the[version property](https://docs.spring.io/spring-boot/docs/current/reference/htmlsingle/#appendix.dependency-versions.properties)defined in the BOM used by the Spring Boot plugin. For example, the name of the JUnit
Jupiter version property in Spring Boot is `junit-jupiter.version`. The mechanism for
changing a dependency version is documented for both[Gradle](https://docs.spring.io/spring-boot/docs/current/gradle-plugin/reference/htmlsingle/#managing-dependencies.dependency-management-plugin.customizing)and[Maven](https://docs.spring.io/spring-boot/docs/current/maven-plugin/reference/htmlsingle/#using.parent-pom).

With Gradle you can override the JUnit Jupiter version by including the following in your`build.gradle` file.

```
ext['junit-jupiter.version'] = '5.10.2'
```

With Maven you can override the JUnit Jupiter version by including the following in your`pom.xml` file.

```
<properties>
    <junit-jupiter.version>5.10.2</junit-jupiter.version>
</properties>
```

### [](#running-tests-console-launcher)4.3. Console Launcher ###

The `[ConsoleLauncher](../api/org.junit.platform.console/org/junit/platform/console/ConsoleLauncher.html)` is a command-line Java application that lets you launch the JUnit
Platform from the console. For example, it can be used to run JUnit Vintage and JUnit
Jupiter tests and print test execution results to the console.

An executable `junit-platform-console-standalone-1.10.2.jar` with all
dependencies included is published in the [Maven Central](https://search.maven.org/) repository under the[junit-platform-console-standalone](https://repo1.maven.org/maven2/org/junit/platform/junit-platform-console-standalone)directory. It includes the following dependencies:

* `junit:junit:4.13.2`

* `org.apiguardian:apiguardian-api:1.1.2`

* `org.hamcrest:hamcrest-core:1.3`

* `org.junit.jupiter:junit-jupiter-api:5.10.2`

* `org.junit.jupiter:junit-jupiter-engine:5.10.2`

* `org.junit.jupiter:junit-jupiter-params:5.10.2`

* `org.junit.platform:junit-platform-commons:1.10.2`

* `org.junit.platform:junit-platform-console:1.10.2`

* `org.junit.platform:junit-platform-engine:1.10.2`

* `org.junit.platform:junit-platform-launcher:1.10.2`

* `org.junit.platform:junit-platform-reporting:1.10.2`

* `org.junit.platform:junit-platform-suite-api:1.10.2`

* `org.junit.platform:junit-platform-suite-commons:1.10.2`

* `org.junit.platform:junit-platform-suite-engine:1.10.2`

* `org.junit.platform:junit-platform-suite:1.10.2`

* `org.junit.vintage:junit-vintage-engine:5.10.2`

* `org.opentest4j:opentest4j:1.3.0`

You can [run](https://docs.oracle.com/javase/tutorial/deployment/jar/run.html) the
standalone `ConsoleLauncher` as shown below.

```
$ java -jar junit-platform-console-standalone-1.10.2.jar execute <OPTIONS>

├─ JUnit Vintage
│  └─ example.JUnit4Tests
│     └─ standardJUnit4Test ✔
└─ JUnit Jupiter
   ├─ StandardTests
   │  ├─ succeedingTest() ✔
   │  └─ skippedTest() ↷ for demonstration purposes
   └─ A special test case
      ├─ Custom test name containing spaces ✔
      ├─ ╯°□°)╯ ✔
      └─ 😱 ✔

Test run finished after 64 ms
[         5 containers found      ]
[         0 containers skipped    ]
[         5 containers started    ]
[         0 containers aborted    ]
[         5 containers successful ]
[         0 containers failed     ]
[         6 tests found           ]
[         1 tests skipped         ]
[         5 tests started         ]
[         0 tests aborted         ]
[         5 tests successful      ]
[         0 tests failed          ]
```

You can also run the standalone `ConsoleLauncher` as shown below (for example, to include
all jars in a directory):

```
$ java -cp classes:testlib/* org.junit.platform.console.ConsoleLauncher <OPTIONS>
```

|   |Exit Code<br/><br/>The `[ConsoleLauncher](../api/org.junit.platform.console/org/junit/platform/console/ConsoleLauncher.html)` exits with a status code of `1` if any containers or tests<br/>failed. If no tests are discovered and the `--fail-if-no-tests` command-line option is<br/>supplied, the `ConsoleLauncher` exits with a status code of `2`. Otherwise, the exit code<br/>is `0`.|
|---|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

#### [](#running-tests-console-launcher-options)4.3.1. Subcommands and Options ####

The `[ConsoleLauncher](../api/org.junit.platform.console/org/junit/platform/console/ConsoleLauncher.html)` provides the following subcommands:

```
Usage: junit [OPTIONS] [COMMAND]
Launches the JUnit Platform for test discovery and execution.
      [@<filename>...]   One or more argument files containing options.
Commands:
  discover  Discover tests
  execute   Execute tests
  engines   List available test engines

For more information, please refer to the JUnit User Guide at
https://junit.org/junit5/docs/current/user-guide/
```

##### [](#discovering-tests)Discovering tests #####

```
Usage: junit discover [OPTIONS]
Discover tests
      [@<filename>...]       One or more argument files containing options.
      --disable-banner       Disable print out of the welcome message.
      --disable-ansi-colors  Disable ANSI colors in output (not supported by all terminals).
  -h, --help                 Display help information.

SELECTORS

      --scan-classpath, --scan-class-path[=PATH]
                             Scan all directories on the classpath or explicit classpath
                               roots. Without arguments, only directories on the system
                               classpath as well as additional classpath entries supplied via
                               -cp (directories and JAR files) are scanned. Explicit classpath
                               roots that are not on the classpath will be silently ignored.
                               This option can be repeated.
      --scan-modules         Scan all resolved modules for test discovery.
  -u, --select-uri=URI       Select a URI for test discovery. This option can be repeated.
  -f, --select-file=FILE     Select a file for test discovery. This option can be repeated.
  -d, --select-directory=DIR Select a directory for test discovery. This option can be
                               repeated.
  -o, --select-module=NAME   Select single module for test discovery. This option can be
                               repeated.
  -p, --select-package=PKG   Select a package for test discovery. This option can be repeated.
  -c, --select-class=CLASS   Select a class for test discovery. This option can be repeated.
  -m, --select-method=NAME   Select a method for test discovery. This option can be repeated.
  -r, --select-resource=RESOURCE
                             Select a classpath resource for test discovery. This option can
                               be repeated.
  -i, --select-iteration=TYPE:VALUE[INDEX(..INDEX)?(,INDEX(..INDEX)?)*]
                             Select iterations for test discovery (e.g. method:com.acme.Foo#m()
                               [1..2]). This option can be repeated.

FILTERS

  -n, --include-classname=PATTERN
                             Provide a regular expression to include only classes whose fully
                               qualified names match. To avoid loading classes unnecessarily,
                               the default pattern only includes class names that begin with
                               "Test" or end with "Test" or "Tests". When this option is
                               repeated, all patterns will be combined using OR semantics.
                               Default: ^(Test.*|.+[.$]Test.*|.*Tests?)$
  -N, --exclude-classname=PATTERN
                             Provide a regular expression to exclude those classes whose fully
                               qualified names match. When this option is repeated, all
                               patterns will be combined using OR semantics.
      --include-package=PKG  Provide a package to be included in the test run. This option can
                               be repeated.
      --exclude-package=PKG  Provide a package to be excluded from the test run. This option
                               can be repeated.
  -t, --include-tag=TAG      Provide a tag or tag expression to include only tests whose tags
                               match. When this option is repeated, all patterns will be
                               combined using OR semantics.
  -T, --exclude-tag=TAG      Provide a tag or tag expression to exclude those tests whose tags
                               match. When this option is repeated, all patterns will be
                               combined using OR semantics.
  -e, --include-engine=ID    Provide the ID of an engine to be included in the test run. This
                               option can be repeated.
  -E, --exclude-engine=ID    Provide the ID of an engine to be excluded from the test run.
                               This option can be repeated.

RUNTIME CONFIGURATION

      -cp, --classpath, --class-path=PATH
                             Provide additional classpath entries -- for example, for adding
                               engines and their dependencies. This option can be repeated.
      --config=KEY=VALUE     Set a configuration parameter for test discovery and execution.
                               This option can be repeated.

CONSOLE OUTPUT

      --color-palette=FILE   Specify a path to a properties file to customize ANSI style of
                               output (not supported by all terminals).
      --single-color         Style test output using only text attributes, no color (not
                               supported by all terminals).
      --details=MODE         Select an output details mode for when tests are executed. Use
                               one of: none, summary, flat, tree, verbose, testfeed. If 'none'
                               is selected, then only the summary and test failures are shown.
                               Default: tree.
      --details-theme=THEME  Select an output details tree theme for when tests are executed.
                               Use one of: ascii, unicode. Default is detected based on
                               default character encoding.

For more information, please refer to the JUnit User Guide at
https://junit.org/junit5/docs/current/user-guide/
```

##### [](#executing-tests)Executing tests #####

```
Usage: junit execute [OPTIONS]
Execute tests
      [@<filename>...]       One or more argument files containing options.
      --disable-banner       Disable print out of the welcome message.
      --disable-ansi-colors  Disable ANSI colors in output (not supported by all terminals).
  -h, --help                 Display help information.

SELECTORS

      --scan-classpath, --scan-class-path[=PATH]
                             Scan all directories on the classpath or explicit classpath
                               roots. Without arguments, only directories on the system
                               classpath as well as additional classpath entries supplied via
                               -cp (directories and JAR files) are scanned. Explicit classpath
                               roots that are not on the classpath will be silently ignored.
                               This option can be repeated.
      --scan-modules         Scan all resolved modules for test discovery.
  -u, --select-uri=URI       Select a URI for test discovery. This option can be repeated.
  -f, --select-file=FILE     Select a file for test discovery. This option can be repeated.
  -d, --select-directory=DIR Select a directory for test discovery. This option can be
                               repeated.
  -o, --select-module=NAME   Select single module for test discovery. This option can be
                               repeated.
  -p, --select-package=PKG   Select a package for test discovery. This option can be repeated.
  -c, --select-class=CLASS   Select a class for test discovery. This option can be repeated.
  -m, --select-method=NAME   Select a method for test discovery. This option can be repeated.
  -r, --select-resource=RESOURCE
                             Select a classpath resource for test discovery. This option can
                               be repeated.
  -i, --select-iteration=TYPE:VALUE[INDEX(..INDEX)?(,INDEX(..INDEX)?)*]
                             Select iterations for test discovery (e.g. method:com.acme.Foo#m()
                               [1..2]). This option can be repeated.

FILTERS

  -n, --include-classname=PATTERN
                             Provide a regular expression to include only classes whose fully
                               qualified names match. To avoid loading classes unnecessarily,
                               the default pattern only includes class names that begin with
                               "Test" or end with "Test" or "Tests". When this option is
                               repeated, all patterns will be combined using OR semantics.
                               Default: ^(Test.*|.+[.$]Test.*|.*Tests?)$
  -N, --exclude-classname=PATTERN
                             Provide a regular expression to exclude those classes whose fully
                               qualified names match. When this option is repeated, all
                               patterns will be combined using OR semantics.
      --include-package=PKG  Provide a package to be included in the test run. This option can
                               be repeated.
      --exclude-package=PKG  Provide a package to be excluded from the test run. This option
                               can be repeated.
  -t, --include-tag=TAG      Provide a tag or tag expression to include only tests whose tags
                               match. When this option is repeated, all patterns will be
                               combined using OR semantics.
  -T, --exclude-tag=TAG      Provide a tag or tag expression to exclude those tests whose tags
                               match. When this option is repeated, all patterns will be
                               combined using OR semantics.
  -e, --include-engine=ID    Provide the ID of an engine to be included in the test run. This
                               option can be repeated.
  -E, --exclude-engine=ID    Provide the ID of an engine to be excluded from the test run.
                               This option can be repeated.

RUNTIME CONFIGURATION

      -cp, --classpath, --class-path=PATH
                             Provide additional classpath entries -- for example, for adding
                               engines and their dependencies. This option can be repeated.
      --config=KEY=VALUE     Set a configuration parameter for test discovery and execution.
                               This option can be repeated.

CONSOLE OUTPUT

      --color-palette=FILE   Specify a path to a properties file to customize ANSI style of
                               output (not supported by all terminals).
      --single-color         Style test output using only text attributes, no color (not
                               supported by all terminals).
      --details=MODE         Select an output details mode for when tests are executed. Use
                               one of: none, summary, flat, tree, verbose, testfeed. If 'none'
                               is selected, then only the summary and test failures are shown.
                               Default: tree.
      --details-theme=THEME  Select an output details tree theme for when tests are executed.
                               Use one of: ascii, unicode. Default is detected based on
                               default character encoding.

REPORTING

      --fail-if-no-tests     Fail and return exit status code 2 if no tests are found.
      --reports-dir=DIR      Enable report output into a specified local directory (will be
                               created if it does not exist).

For more information, please refer to the JUnit User Guide at
https://junit.org/junit5/docs/current/user-guide/
```

##### [](#listing-test-engines)Listing test engines #####

```
Usage: junit engines [OPTIONS]
List available test engines
      [@<filename>...]   One or more argument files containing options.
      --disable-banner   Disable print out of the welcome message.
      --disable-ansi-colors
                         Disable ANSI colors in output (not supported by all terminals).
  -h, --help             Display help information.

For more information, please refer to the JUnit User Guide at
https://junit.org/junit5/docs/current/user-guide/
```

#### [](#running-tests-console-launcher-argument-files)4.3.2. Argument Files (@-files) ####

On some platforms you may run into system limitations on the length of a command line
when creating a command line with lots of options or with long arguments.

Since version 1.3, the `ConsoleLauncher` supports *argument files*, also known as*@-files*. Argument files are files that themselves contain arguments to be passed to the
command. When the underlying [picocli](https://github.com/remkop/picocli) command line
parser encounters an argument beginning with the character `@`, it expands the contents
of that file into the argument list.

The arguments within a file can be separated by spaces or newlines. If an argument
contains embedded whitespace, the whole argument should be wrapped in double or single
quotes — for example, `"-f=My Files/Stuff.java"`.

If the argument file does not exist or cannot be read, the argument will be treated
literally and will not be removed. This will likely result in an "unmatched argument"
error message. You can troubleshoot such errors by executing the command with the`picocli.trace` system property set to `DEBUG`.

Multiple *@-files* may be specified on the command line. The specified path may be
relative to the current directory or absolute.

You can pass a real parameter with an initial `@` character by escaping it with an
additional `@` symbol. For example, `@@somearg` will become `@somearg` and will not be
subject to expansion.

#### [](#running-tests-console-launcher-color-customization)4.3.3. Color customization ####

The colors used in the output of the `[ConsoleLauncher](../api/org.junit.platform.console/org/junit/platform/console/ConsoleLauncher.html)` can be customized.
The option `--single-color` will apply a built-in monochrome style, while`--color-palette` will accept a properties file to override the[ANSI SGR](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors) color styling.
The properties file below demonstrates the default style:

```
SUCCESSFUL = 32
ABORTED = 33
FAILED = 31
SKIPPED = 35
CONTAINER = 35
TEST = 34
DYNAMIC = 35
REPORTED = 37
```

### [](#running-tests-junit-platform-runner)4.4. Using JUnit 4 to run the JUnit Platform ###

|   |The `JUnitPlatform` runner has been deprecated<br/><br/>The `JUnitPlatform` runner was developed by the JUnit team as an interim solution for<br/>running test suites and tests on the JUnit Platform in a JUnit 4 environment.<br/><br/>In recent years, all mainstream build tools and IDEs provide built-in support for running<br/>tests directly on the JUnit Platform.<br/><br/>In addition, the introduction of `@Suite` support provided by the`junit-platform-suite-engine` module makes the `JUnitPlatform` runner obsolete. See[JUnit Platform Suite Engine](#junit-platform-suite-engine) for details.<br/><br/>The `JUnitPlatform` runner and `@UseTechnicalNames` annotation have therefore been<br/>deprecated in JUnit Platform 1.8 and will be removed in JUnit Platform 2.0.<br/><br/>If you are using the `JUnitPlatform` runner, please migrate to the `@Suite` support.|
|---|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

The `JUnitPlatform` runner is a JUnit 4 based `Runner` which enables you to run any test
whose programming model is supported on the JUnit Platform in a JUnit 4 environment — for example, a JUnit Jupiter test class.

Annotating a class with `@RunWith(JUnitPlatform.class)` allows it to be run with IDEs and
build systems that support JUnit 4 but do not yet support the JUnit Platform directly.

|   |Since the JUnit Platform has features that JUnit 4 does not have, the runner is<br/>only able to support a subset of the JUnit Platform functionality, especially with regard<br/>to reporting (see [Display Names vs. Technical Names](#running-tests-junit-platform-runner-technical-names)).|
|---|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

#### [](#running-tests-junit-platform-runner-setup)4.4.1. Setup ####

You need the following artifacts and their dependencies on the classpath. See[Dependency Metadata](#dependency-metadata) for details regarding group IDs, artifact IDs, and versions.

##### [](#running-tests-junit-platform-runner-setup-explicit-dependencies)Explicit Dependencies #####

* `junit-platform-runner` in *test* scope: location of the `JUnitPlatform` runner

* `junit-4.13.2.jar` in *test* scope: to run tests using JUnit 4

* `junit-jupiter-api` in *test* scope: API for writing tests using JUnit Jupiter,
  including `@Test`, etc.

* `junit-jupiter-engine` in *test runtime* scope: implementation of the `TestEngine` API
  for JUnit Jupiter

##### [](#running-tests-junit-platform-runner-setup-transitive-dependencies)Transitive Dependencies #####

* `junit-platform-suite-api` in *test* scope

* `junit-platform-suite-commons` in *test* scope

* `junit-platform-launcher` in *test* scope

* `junit-platform-engine` in *test* scope

* `junit-platform-commons` in *test* scope

* `opentest4j` in *test* scope

#### [](#running-tests-junit-platform-runner-technical-names)4.4.2. Display Names vs. Technical Names ####

To define a custom *display name* for the class run via `@RunWith(JUnitPlatform.class)`annotate the class with `@SuiteDisplayName` and provide a custom value.

By default, *display names* will be used for test artifacts; however, when the`JUnitPlatform` runner is used to execute tests with a build tool such as Gradle or
Maven, the generated test report often needs to include the *technical names* of test
artifacts — for example, fully qualified class names — instead of shorter display names
like the simple name of a test class or a custom display name containing special
characters. To enable technical names for reporting purposes, declare the`@UseTechnicalNames` annotation alongside `@RunWith(JUnitPlatform.class)`.

Note that the presence of `@UseTechnicalNames` overrides any custom display name
configured via `@SuiteDisplayName`.

#### [](#running-tests-junit-platform-runner-single-test)4.4.3. Single Test Class ####

One way to use the `JUnitPlatform` runner is to annotate a test class with`@RunWith(JUnitPlatform.class)` directly. Please note that the test methods in the
following example are annotated with `org.junit.jupiter.api.Test` (JUnit Jupiter), not`org.junit.Test` (JUnit 4). Moreover, in this case the test class must be `public`;
otherwise, some IDEs and build tools might not recognize it as a JUnit 4 test class.

```
import static org.junit.jupiter.api.Assertions.fail;

import org.junit.jupiter.api.Test;
import org.junit.runner.RunWith;

@RunWith(org.junit.platform.runner.JUnitPlatform.class)
public class JUnitPlatformClassDemo {

    @Test
    void succeedingTest() {
        /* no-op */
    }

    @Test
    void failingTest() {
        fail("Failing for failing's sake.");
    }

}
```

#### [](#running-tests-junit-platform-runner-test-suite)4.4.4. Test Suite ####

If you have multiple test classes you can create a test suite as can be seen in the
following example.

```
import org.junit.platform.suite.api.SelectPackages;
import org.junit.platform.suite.api.SuiteDisplayName;
import org.junit.runner.RunWith;

@RunWith(org.junit.platform.runner.JUnitPlatform.class)
@SuiteDisplayName("JUnit Platform Suite Demo")
@SelectPackages("example")
public class JUnitPlatformSuiteDemo {
}
```

The `JUnitPlatformSuiteDemo` will discover and run all tests in the `example` package and
its subpackages. By default, it will only include test classes whose names either begin
with `Test` or end with `Test` or `Tests`.

|   |Additional Configuration Options<br/><br/>There are more configuration options for discovering and filtering tests than just`@SelectPackages`. Please consult the Javadoc of the `[org.junit.platform.suite.api](../api/org.junit.platform.suite.api/org/junit/platform/suite/api/package-summary.html)` package for<br/>further details.|
|---|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

|   |Test classes and suites annotated with `@RunWith(JUnitPlatform.class)`**cannot** be executed directly on the JUnit Platform (or as a "JUnit 5" test as<br/>documented in some IDEs). Such classes and suites can only be executed using JUnit 4<br/>infrastructure.|
|---|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

### [](#running-tests-config-params)4.5. Configuration Parameters ###

In addition to instructing the platform which test classes and test engines to include,
which packages to scan, etc., it is sometimes necessary to provide additional custom
configuration parameters that are specific to a particular test engine, listener, or
registered extension. For example, the JUnit Jupiter `TestEngine` supports *configuration
parameters* for the following use cases.

* [Changing the Default Test Instance Lifecycle](#writing-tests-test-instance-lifecycle-changing-default)

* [Enabling Automatic Extension Detection](#extensions-registration-automatic-enabling)

* [Deactivating Conditions](#extensions-conditions-deactivation)

* [Setting the Default Display Name Generator](#writing-tests-display-name-generator-default)

*Configuration Parameters* are text-based key-value pairs that can be supplied to test
engines running on the JUnit Platform via one of the following mechanisms.

1. The `configurationParameter()` and `configurationParameters()` methods in the`LauncherDiscoveryRequestBuilder` which is used to build a request supplied to the[`Launcher` API](#launcher-api). When running tests via one of the tools provided
   by the JUnit Platform you can specify configuration parameters as follows:

   * [Console Launcher](#running-tests-console-launcher): use the `--config`command-line option.

   * [Gradle](#running-tests-build-gradle-config-params): use the`systemProperty` or `systemProperties` DSL.

   * [Maven Surefire provider](#running-tests-build-maven-config-params): use the`configurationParameters` property.

2. JVM system properties.

3. The JUnit Platform configuration file: a file named `junit-platform.properties` in the
   root of the class path that follows the syntax rules for a Java `Properties` file.

|   |Configuration parameters are looked up in the exact order defined above.<br/>Consequently, configuration parameters supplied directly to the `Launcher` take<br/>precedence over those supplied via system properties and the configuration file.<br/>Similarly, configuration parameters supplied via system properties take precedence over<br/>those supplied via the configuration file.|
|---|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

#### [](#running-tests-config-params-deactivation-pattern)4.5.1. Pattern Matching Syntax ####

This section describes the pattern matching syntax that is applied to the *configuration
parameters* used for the following features.

* [Deactivating Conditions](#extensions-conditions-deactivation)

* [Deactivating a TestExecutionListener](#launcher-api-listeners-custom-deactivation)

* [Stack Trace Pruning](#stacktrace-pruning)

If the value for the given *configuration parameter* consists solely of an asterisk
(`*`), the pattern will match against all candidate classes. Otherwise, the value
will be treated as a comma-separated list of patterns where each pattern will be matched
against the fully qualified class name (*FQCN*) of each candidate class. Any dot (`.`) in
a pattern will match against a dot (`.`) or a dollar sign (`$`) in a FQCN. Any asterisk
(`*`) will match against one or more characters in a FQCN. All other characters in a
pattern will be matched one-to-one against a FQCN.

Examples:

* `*`: matches all candidate classes.

* `org.junit.*`: matches all candidate classes under the `org.junit` base package and
  any of its subpackages.

* `*.MyCustomImpl`: matches every candidate class whose simple class name is exactly`MyCustomImpl`.

* `*System*`: matches every candidate class whose FQCN contains `System`.

* `*System*, *Unit*`: matches every candidate class whose FQCN contains`System` or `Unit`.

* `org.example.MyCustomImpl`: matches the candidate class whose FQCN is exactly`org.example.MyCustomImpl`.

* `org.example.MyCustomImpl, org.example.TheirCustomImpl`: matches candidate classes whose
  FQCN is exactly `org.example.MyCustomImpl` or `org.example.TheirCustomImpl`.

### [](#running-tests-tags)4.6. Tags ###

Tags are a JUnit Platform concept for marking and filtering tests. The programming model
for adding tags to containers and tests is defined by the testing framework. For example,
in JUnit Jupiter based tests, the `@Tag` annotation (see[Tagging and Filtering](#writing-tests-tagging-and-filtering)) should be used. For JUnit 4 based tests, the
Vintage engine maps `@Category` annotations to tags (see[Categories Support](#migrating-from-junit4-categories-support)). Other testing frameworks may define their
own annotation or other means for users to specify tags.

#### [](#running-tests-tag-syntax-rules)4.6.1. Syntax Rules for Tags ####

Regardless how a tag is specified, the JUnit Platform enforces the following rules:

* A tag must not be `null` or *blank*.

* A *trimmed* tag must not contain whitespace.

* A *trimmed* tag must not contain ISO control characters.

* A *trimmed* tag must not contain any of the following *reserved characters*.

  * `,`: *comma*

  * `(`: *left parenthesis*

  * `)`: *right parenthesis*

  * `&`: *ampersand*

  * `|`: *vertical bar*

  * `!`: *exclamation point*

|   |In the above context, "trimmed" means that leading and trailing whitespace<br/>characters have been removed.|
|---|------------------------------------------------------------------------------------------------------------|

#### [](#running-tests-tag-expressions)4.6.2. Tag Expressions ####

Tag expressions are boolean expressions with the operators `!`, `&` and `|`. In addition,`(` and `)` can be used to adjust for operator precedence.

Two special expressions are supported, `any()` and `none()`, which select all tests *with*any tags at all, and all tests *without* any tags, respectively.
These special expressions may be combined with other expressions just like normal tags.

|Operator|Meaning|Associativity|
|--------|-------|-------------|
|  `!`   |  not  |    right    |
|  `&`   |  and  |    left     |
|  `|`   |  or   |    left     |

If you are tagging your tests across multiple dimensions, tag expressions help you to
select which tests to execute. When tagging by test type (e.g., *micro*, *integration*,*end-to-end*) and feature (e.g., **product**, **catalog**, **shipping**), the following tag
expressions can be useful.

|                Tag Expression                |                             Selection                             |
|----------------------------------------------|-------------------------------------------------------------------|
|                  `product`                   |                     all tests for **product**                     |
|             `catalog | shipping`             |     all tests for **catalog** plus all tests for **shipping**     |
|             `catalog & shipping`             |all tests for the intersection between **catalog** and **shipping**|
|           `product & !end-to-end`            |     all tests for **product**, but not the *end-to-end* tests     |
|`(micro | integration) & (product | shipping)`|all *micro* or *integration* tests for **product** or **shipping** |

### [](#running-tests-capturing-output)4.7. Capturing Standard Output/Error ###

Since version 1.3, the JUnit Platform provides opt-in support for capturing output
printed to `System.out` and `System.err`. To enable it, set the`junit.platform.output.capture.stdout` and/or `junit.platform.output.capture.stderr`[configuration parameter](#running-tests-config-params) to `true`. In addition, you may
configure the maximum number of buffered bytes to be used per executed test or container
using `junit.platform.output.capture.maxBuffer`.

If enabled, the JUnit Platform captures the corresponding output and publishes it as a
report entry using the `stdout` or `stderr` keys to all registered`[TestExecutionListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/TestExecutionListener.html)` instances immediately before reporting the test or container as
finished.

Please note that the captured output will only contain output emitted by the thread that
was used to execute a container or test. Any output by other threads will be omitted
because particularly when[executing tests in parallel](#writing-tests-parallel-execution) it would be impossible
to attribute it to a specific test or container.

### [](#running-tests-listeners)4.8. Using Listeners and Interceptors ###

The JUnit Platform provides the following listener APIs that allow JUnit, third parties,
and custom user code to react to events fired at various points during the discovery and
execution of a `TestPlan`.

* `[LauncherSessionListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/LauncherSessionListener.html)`: receives events when a `[LauncherSession](../api/org.junit.platform.launcher/org/junit/platform/launcher/LauncherSession.html)` is opened and
  closed.

* `[LauncherInterceptor](../api/org.junit.platform.launcher/org/junit/platform/launcher/LauncherInterceptor.html)`: intercepts test discovery and execution in the context of a`LauncherSession`.

* `[LauncherDiscoveryListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/LauncherDiscoveryListener.html)`: receives events that occur during test discovery.

* `[TestExecutionListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/TestExecutionListener.html)`: receives events that occur during test execution.

The `LauncherSessionListener` API is typically implemented by build tools or IDEs and
registered automatically for you in order to support some feature of the build tool or IDE.

The `LauncherDiscoveryListener` and `TestExecutionListener` APIs are often implemented in
order to produce some form of report or to display a graphical representation of the test
plan in an IDE. Such listeners may be implemented and automatically registered by a build
tool or IDE, or they may be included in a third-party library – potentially registered
for you automatically. You can also implement and register your own listeners.

For details on registering and configuring listeners, see the following sections of this
guide.

* [Registering a LauncherSessionListener](#launcher-api-launcher-session-listeners-custom)

* [Registering a LauncherInterceptor](#launcher-api-launcher-interceptors-custom)

* [Registering a LauncherDiscoveryListener](#launcher-api-launcher-discovery-listeners-custom)

* [Registering a TestExecutionListener](#launcher-api-listeners-custom)

* [Configuring a TestExecutionListener](#launcher-api-listeners-config)

* [Deactivating a TestExecutionListener](#launcher-api-listeners-custom-deactivation)

The JUnit Platform provides the following listeners which you may wish to use with your
test suite.

[JUnit Platform Reporting](#junit-platform-reporting)

`[LegacyXmlReportGeneratingListener](../api/org.junit.platform.reporting/org/junit/platform/reporting/legacy/xml/LegacyXmlReportGeneratingListener.html)` can be used via the[Console Launcher](#running-tests-console-launcher) or registered manually to generate XML reports
compatible with the de facto standard for JUnit 4 based test reports.

`[OpenTestReportGeneratingListener](../api/org.junit.platform.reporting/org/junit/platform/reporting/open/xml/OpenTestReportGeneratingListener.html)` generates an XML report in the event-based format
specified by [Open Test Reporting](https://github.com/ota4j-team/open-test-reporting). It is auto-registered and can be enabled and
configured via [Configuration Parameters](#running-tests-config-params).

See [JUnit Platform Reporting](#junit-platform-reporting) for details.

[Flight Recorder Support](#running-tests-listeners-flight-recorder)

`FlightRecordingExecutionListener` and `FlightRecordingDiscoveryListener` that generate
Java Flight Recorder events during test discovery and execution.

`[LoggingListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/listeners/LoggingListener.html)`

`TestExecutionListener` for logging informational messages for all events via a`BiConsumer` that consumes `Throwable` and `Supplier<String>`.

`[SummaryGeneratingListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/listeners/SummaryGeneratingListener.html)`

`TestExecutionListener` that generates a summary of the test execution which can be
printed via a `PrintWriter`.

`[UniqueIdTrackingListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/listeners/UniqueIdTrackingListener.html)`

`TestExecutionListener` that that tracks the unique IDs of all tests that were skipped
or executed during the execution of the `TestPlan` and generates a file containing the
unique IDs once execution of the `TestPlan` has finished.

#### [](#running-tests-listeners-flight-recorder)4.8.1. Flight Recorder Support ####

Since version 1.7, the JUnit Platform provides opt-in support for generating Flight
Recorder events. [JEP 328](https://openjdk.java.net/jeps/328) describes the Java Flight
Recorder (JFR) as:

|   |Flight Recorder records events originating from applications, the JVM and the OS.<br/>Events are stored in a single file that can be attached to bug reports and examined by<br/>support engineers, allowing after-the-fact analysis of issues in the period leading up<br/>to a problem.|
|---|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

In order to record Flight Recorder events generated while running tests, you need to:

1. Ensure that you are using either Java 8 Update 262 or higher or Java 11 or later.

2. Provide the `org.junit.platform.jfr` module (`junit-platform-jfr-1.10.2.jar`)
   on the class-path or module-path at test runtime.

3. Start flight recording when launching a test run. Flight Recorder can be started via
   java command line option:

   ```
   -XX:StartFlightRecording:filename=...
   ```

Please consult the manual of your build tool for the appropriate commands.

To analyze the recorded events, use the[jfr](https://docs.oracle.com/en/java/javase/14/docs/specs/man/jfr.html)command line tool shipped with recent JDKs or open the recording file with[JDK Mission Control](https://jdk.java.net/jmc/).

|   |Flight Recorder support is currently an *experimental* feature. You’re invited to<br/>give it a try and provide feedback to the JUnit team so they can improve and eventually[promote](#api-evolution) this feature.|
|---|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

### [](#stacktrace-pruning)4.9. Stack Trace Pruning ###

Since version 1.10, the JUnit Platform provides built-in support for pruning stack traces
produced by failing tests. This feature is enabled by default but can be disabled by
setting the `junit.platform.stacktrace.pruning.enabled` *configuration parameter* to`false`.

When enabled, all calls from the `org.junit`, `jdk.internal.reflect`, and `sun.reflect`packages are removed from the stack trace, unless the calls occur after the test itself
or any of its ancestors. For that reason, calls to `[org.junit.jupiter.api.Assertions](../api/org.junit.jupiter.api/org/junit/jupiter/api/Assertions.html)` or `[org.junit.jupiter.api.Assumptions](../api/org.junit.jupiter.api/org/junit/jupiter/api/Assumptions.html)` will
never be excluded.

In addition, all elements prior to and including the first call from the JUnit Platform
Launcher will be removed.

[](#extensions)5. Extension Model
----------

### [](#extensions-overview)5.1. Overview ###

In contrast to the competing `Runner`, `TestRule`, and `MethodRule` extension points in
JUnit 4, the JUnit Jupiter extension model consists of a single, coherent concept: the`Extension` API. Note, however, that `Extension` itself is just a marker interface.

### [](#extensions-registration)5.2. Registering Extensions ###

Extensions can be registered *declaratively* via[`@ExtendWith`](#extensions-registration-declarative), *programmatically* via[`@RegisterExtension`](#extensions-registration-programmatic), or *automatically* via
Java’s [`ServiceLoader`](#extensions-registration-automatic) mechanism.

#### [](#extensions-registration-declarative)5.2.1. Declarative Extension Registration ####

Developers can register one or more extensions *declaratively* by annotating a test
interface, test class, test method, or custom *[composed
annotation](#writing-tests-meta-annotations)* with `@ExtendWith(…​)` and supplying class references for the extensions to
register. As of JUnit Jupiter 5.8, `@ExtendWith` may also be declared on fields or on
parameters in test class constructors, in test methods, and in `@BeforeAll`, `@AfterAll`,`@BeforeEach`, and `@AfterEach` lifecycle methods.

For example, to register a `WebServerExtension` for a particular test method, you would
annotate the test method as follows. We assume the `WebServerExtension` starts a local web
server and injects the server’s URL into parameters annotated with `@WebServerUrl`.

```
@Test
@ExtendWith(WebServerExtension.class)
void getProductList(@WebServerUrl String serverUrl) {
    WebClient webClient = new WebClient();
    // Use WebClient to connect to web server using serverUrl and verify response
    assertEquals(200, webClient.get(serverUrl + "/products").getResponseStatus());
}
```

To register the `WebServerExtension` for all tests in a particular class and its
subclasses, you would annotate the test class as follows.

```
@ExtendWith(WebServerExtension.class)
class MyTests {
    // ...
}
```

Multiple extensions can be registered together like this:

```
@ExtendWith({ DatabaseExtension.class, WebServerExtension.class })
class MyFirstTests {
    // ...
}
```

As an alternative, multiple extensions can be registered separately like this:

```
@ExtendWith(DatabaseExtension.class)
@ExtendWith(WebServerExtension.class)
class MySecondTests {
    // ...
}
```

|   |Extension Registration Order<br/><br/>Extensions registered declaratively via `@ExtendWith` at the class level, method level, or<br/>parameter level will be executed in the order in which they are declared in the source<br/>code. For example, the execution of tests in both `MyFirstTests` and `MySecondTests` will<br/>be extended by the `DatabaseExtension` and `WebServerExtension`, **in exactly that order**.|
|---|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

If you wish to combine multiple extensions in a reusable way, you can define a custom*[composed annotation](#writing-tests-meta-annotations)* and use `@ExtendWith` as a*meta-annotation* as in the following code listing. Then `@DatabaseAndWebServerExtension`can be used in place of `@ExtendWith({ DatabaseExtension.class, WebServerExtension.class })`.

```
@Target({ ElementType.TYPE, ElementType.METHOD })
@Retention(RetentionPolicy.RUNTIME)
@ExtendWith({ DatabaseExtension.class, WebServerExtension.class })
public @interface DatabaseAndWebServerExtension {
}
```

The above examples demonstrate how `@ExtendWith` can be applied at the class level or at
the method level; however, for certain use cases it makes sense for an extension to be
registered declaratively at the field or parameter level. Consider a`RandomNumberExtension` which generates random numbers that can be injected into a field or
via a parameter in a constructor, test method, or lifecycle method. If the extension
provides a `@Random` annotation that is meta-annotated with`@ExtendWith(RandomNumberExtension.class)` (see listing below), the extension can be used
transparently as in the following `RandomNumberDemo` example.

```
@Target({ ElementType.FIELD, ElementType.PARAMETER })
@Retention(RetentionPolicy.RUNTIME)
@ExtendWith(RandomNumberExtension.class)
public @interface Random {
}
```

```
class RandomNumberDemo {

    // Use static randomNumber0 field anywhere in the test class,
    // including @BeforeAll or @AfterEach lifecycle methods.
    @Random
    private static Integer randomNumber0;

    // Use randomNumber1 field in test methods and @BeforeEach
    // or @AfterEach lifecycle methods.
    @Random
    private int randomNumber1;

    RandomNumberDemo(@Random int randomNumber2) {
        // Use randomNumber2 in constructor.
    }

    @BeforeEach
    void beforeEach(@Random int randomNumber3) {
        // Use randomNumber3 in @BeforeEach method.
    }

    @Test
    void test(@Random int randomNumber4) {
        // Use randomNumber4 in test method.
    }

}
```

The following code listing provides an example of how one might choose to implement such a`RandomNumberExtension`. This implementation works for the use cases in`RandomNumberDemo`; however, it may not prove robust enough to cover all use cases — for
example, the random number generation support is limited to integers; it uses`java.util.Random` instead of `java.security.SecureRandom`; etc. In any case, it is
important to note which extension APIs are implemented and for what reasons.

Specifically, `RandomNumberExtension` implements the following extension APIs:

* `BeforeAllCallback`: to support static field injection

* `BeforeEachCallback`: to support non-static field injection

* `ParameterResolver`: to support constructor and method injection

|   |Ideally, the `RandomNumberExtension` would implement `TestInstancePostProcessor` instead<br/>of `BeforeEachCallback` in order to support non-static field injection immediately after<br/>the test class has been instantiated.<br/><br/>However, JUnit Jupiter currently does not allow a `TestInstancePostProcessor` to be<br/>registered via `@ExtendWith` on a non-static field (see[issue 3437](https://github.com/junit-team/junit5/issues/3437)). In light of that, the `RandomNumberExtension`implements `BeforeEachCallback` as an alternative approach.|
|---|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

```
import static org.junit.platform.commons.support.AnnotationSupport.findAnnotatedFields;

import java.lang.reflect.Field;
import java.util.function.Predicate;

import org.junit.jupiter.api.extension.BeforeAllCallback;
import org.junit.jupiter.api.extension.BeforeEachCallback;
import org.junit.jupiter.api.extension.ExtensionContext;
import org.junit.jupiter.api.extension.ParameterContext;
import org.junit.jupiter.api.extension.ParameterResolver;
import org.junit.platform.commons.support.ModifierSupport;

class RandomNumberExtension
        implements BeforeAllCallback, BeforeEachCallback, ParameterResolver {

    private final java.util.Random random = new java.util.Random(System.nanoTime());

    /**
     * Inject a random integer into static fields that are annotated with
     * {@code @Random} and can be assigned an integer value.
     */
    @Override
    public void beforeAll(ExtensionContext context) {
        Class<?> testClass = context.getRequiredTestClass();
        injectFields(testClass, null, ModifierSupport::isStatic);
    }

    /**
     * Inject a random integer into non-static fields that are annotated with
     * {@code @Random} and can be assigned an integer value.
     */
    @Override
    public void beforeEach(ExtensionContext context) {
        Class<?> testClass = context.getRequiredTestClass();
        Object testInstance = context.getRequiredTestInstance();
        injectFields(testClass, testInstance, ModifierSupport::isNotStatic);
    }

    /**
     * Determine if the parameter is annotated with {@code @Random} and can be
     * assigned an integer value.
     */
    @Override
    public boolean supportsParameter(ParameterContext pc, ExtensionContext ec) {
        return pc.isAnnotated(Random.class) && isInteger(pc.getParameter().getType());
    }

    /**
     * Resolve a random integer.
     */
    @Override
    public Integer resolveParameter(ParameterContext pc, ExtensionContext ec) {
        return this.random.nextInt();
    }

    private void injectFields(Class<?> testClass, Object testInstance,
            Predicate<Field> predicate) {

        predicate = predicate.and(field -> isInteger(field.getType()));
        findAnnotatedFields(testClass, Random.class, predicate)
            .forEach(field -> {
                try {
                    field.setAccessible(true);
                    field.set(testInstance, this.random.nextInt());
                }
                catch (Exception ex) {
                    throw new RuntimeException(ex);
                }
            });
    }

    private static boolean isInteger(Class<?> type) {
        return type == Integer.class || type == int.class;
    }

}
```

|   |Extension Registration Order for `@ExtendWith` on Fields<br/><br/>Extensions registered declaratively via `@ExtendWith` on fields will be ordered relative<br/>to `@RegisterExtension` fields and other `@ExtendWith` fields using an algorithm that is<br/>deterministic but intentionally nonobvious. However, `@ExtendWith` fields can be ordered<br/>using the `@Order` annotation. See the [Extension Registration Order](#extensions-registration-programmatic-order) tip for `@RegisterExtension` fields for details.|
|---|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

|   |`@ExtendWith` fields may be either `static` or non-static. The documentation on[Static Fields](#extensions-registration-programmatic-static-fields) and[Instance Fields](#extensions-registration-programmatic-instance-fields) for`@RegisterExtension` fields also applies to `@ExtendWith` fields.|
|---|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

#### [](#extensions-registration-programmatic)5.2.2. Programmatic Extension Registration ####

Developers can register extensions *programmatically* by annotating fields in test classes
with `[@RegisterExtension](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/RegisterExtension.html)`.

When an extension is registered *declaratively* via[`@ExtendWith`](#extensions-registration-declarative), it can typically only be configured
via annotations. In contrast, when an extension is registered via `@RegisterExtension`, it
can be configured *programmatically* — for example, in order to pass arguments to the
extension’s constructor, a static factory method, or a builder API.

|   |Extension Registration Order<br/><br/>By default, extensions registered programmatically via `@RegisterExtension` or<br/>declaratively via `@ExtendWith` on fields will be ordered using an algorithm that is<br/>deterministic but intentionally nonobvious. This ensures that subsequent runs of a test<br/>suite execute extensions in the same order, thereby allowing for repeatable builds.<br/>However, there are times when extensions need to be registered in an explicit order. To<br/>achieve that, annotate `@RegisterExtension` fields or `@ExtendWith` fields with `[@Order](../api/org.junit.jupiter.api/org/junit/jupiter/api/Order.html)`.<br/><br/>Any `@RegisterExtension` field or `@ExtendWith` field not annotated with `@Order` will be<br/>ordered using the *default* order which has a value of `Integer.MAX_VALUE / 2`. This<br/>allows `@Order` annotated extension fields to be explicitly ordered before or after<br/>non-annotated extension fields. Extensions with an explicit order value less than the<br/>default order value will be registered before non-annotated extensions. Similarly,<br/>extensions with an explicit order value greater than the default order value will be<br/>registered after non-annotated extensions. For example, assigning an extension an explicit<br/>order value that is greater than the default order value allows *before* callback<br/>extensions to be registered last and *after* callback extensions to be registered first,<br/>relative to other programmatically registered extensions.|
|---|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

|   |`@RegisterExtension` fields must not be `null` (at evaluation time) but may be<br/>either `static` or non-static.|
|---|-----------------------------------------------------------------------------------------------------------------|

##### [](#extensions-registration-programmatic-static-fields)Static Fields #####

If a `@RegisterExtension` field is `static`, the extension will be registered after
extensions that are registered at the class level via `@ExtendWith`. Such *static
extensions* are not limited in which extension APIs they can implement. Extensions
registered via static fields may therefore implement class-level and instance-level
extension APIs such as `BeforeAllCallback`, `AfterAllCallback`,`TestInstancePostProcessor`, and `TestInstancePreDestroyCallback` as well as method-level
extension APIs such as `BeforeEachCallback`, etc.

In the following example, the `server` field in the test class is initialized
programmatically by using a builder pattern supported by the `WebServerExtension`. The
configured `WebServerExtension` will be automatically registered as an extension at the
class level — for example, in order to start the server before all tests in the class
and then stop the server after all tests in the class have completed. In addition, static
lifecycle methods annotated with `@BeforeAll` or `@AfterAll` as well as `@BeforeEach`,`@AfterEach`, and `@Test` methods can access the instance of the extension via the`server` field if necessary.

Registering an extension via a static field in Java

```
class WebServerDemo {

    @RegisterExtension
    static WebServerExtension server = WebServerExtension.builder()
        .enableSecurity(false)
        .build();

    @Test
    void getProductList() {
        WebClient webClient = new WebClient();
        String serverUrl = server.getServerUrl();
        // Use WebClient to connect to web server using serverUrl and verify response
        assertEquals(200, webClient.get(serverUrl + "/products").getResponseStatus());
    }

}
```

###### [](#extensions-registration-programmatic-static-fields-kotlin)Static Fields in Kotlin ######

The Kotlin programming language does not have the concept of a `static` field. However,
the compiler can be instructed to generate a `private static` field using the `@JvmStatic`annotation in Kotlin. If you want the Kotlin compiler to generate a `public static` field,
you can use the `@JvmField` annotation instead.

The following example is a version of the `WebServerDemo` from the previous section that
has been ported to Kotlin.

Registering an extension via a static field in Kotlin

```
class KotlinWebServerDemo {

    companion object {
        @JvmStatic
        @RegisterExtension
        val server = WebServerExtension.builder()
            .enableSecurity(false)
            .build()
    }

    @Test
    fun getProductList() {
        // Use WebClient to connect to web server using serverUrl and verify response
        val webClient = WebClient()
        val serverUrl = server.serverUrl
        assertEquals(200, webClient.get("$serverUrl/products").responseStatus)
    }
}
```

##### [](#extensions-registration-programmatic-instance-fields)Instance Fields #####

If a `@RegisterExtension` field is non-static (i.e., an instance field), the extension
will be registered after the test class has been instantiated and after each registered`TestInstancePostProcessor` has been given a chance to post-process the test instance
(potentially injecting the instance of the extension to be used into the annotated
field). Thus, if such an *instance extension* implements class-level or instance-level
extension APIs such as `BeforeAllCallback`, `AfterAllCallback`, or`TestInstancePostProcessor`, those APIs will not be honored. By default, an instance
extension will be registered *after* extensions that are registered at the method level
via `@ExtendWith`; however, if the test class is configured with`@TestInstance(Lifecycle.PER_CLASS)` semantics, an instance extension will be registered*before* extensions that are registered at the method level via `@ExtendWith`.

In the following example, the `docs` field in the test class is initialized
programmatically by invoking a custom `lookUpDocsDir()` method and supplying the result
to the static `forPath()` factory method in the `DocumentationExtension`. The configured`DocumentationExtension` will be automatically registered as an extension at the method
level. In addition, `@BeforeEach`, `@AfterEach`, and `@Test` methods can access the
instance of the extension via the `docs` field if necessary.

An extension registered via an instance field

```
class DocumentationDemo {

    static Path lookUpDocsDir() {
        // return path to docs dir
    }

    @RegisterExtension
    DocumentationExtension docs = DocumentationExtension.forPath(lookUpDocsDir());

    @Test
    void generateDocumentation() {
        // use this.docs ...
    }
}
```

#### [](#extensions-registration-automatic)5.2.3. Automatic Extension Registration ####

In addition to [declarative extension registration](#extensions-registration-declarative)and [programmatic extension registration](#extensions-registration-programmatic) support
using annotations, JUnit Jupiter also supports *global extension registration* via Java’s`[ServiceLoader](https://docs.oracle.com/en/java/javase/11/docs/api/java.base/java/util/ServiceLoader.html)` mechanism, allowing third-party extensions to be auto-detected and
automatically registered based on what is available in the classpath.

Specifically, a custom extension can be registered by supplying its fully qualified class
name in a file named `org.junit.jupiter.api.extension.Extension` within the`/META-INF/services` folder in its enclosing JAR file.

##### [](#extensions-registration-automatic-enabling)Enabling Automatic Extension Detection #####

Auto-detection is an advanced feature and is therefore not enabled by default. To enable
it, set the `junit.jupiter.extensions.autodetection.enabled` *configuration parameter* to`true`. This can be supplied as a JVM system property, as a *configuration parameter* in
the `LauncherDiscoveryRequest` that is passed to the `Launcher`, or via the JUnit Platform
configuration file (see [Configuration Parameters](#running-tests-config-params) for details).

For example, to enable auto-detection of extensions, you can start your JVM with the
following system property.

`-Djunit.jupiter.extensions.autodetection.enabled=true`

When auto-detection is enabled, extensions discovered via the `[ServiceLoader](https://docs.oracle.com/en/java/javase/11/docs/api/java.base/java/util/ServiceLoader.html)` mechanism
will be added to the extension registry after JUnit Jupiter’s global extensions (e.g.,
support for `TestInfo`, `TestReporter`, etc.).

#### [](#extensions-registration-inheritance)5.2.4. Extension Inheritance ####

Registered extensions are inherited within test class hierarchies with top-down
semantics. Similarly, extensions registered at the class-level are inherited at the
method-level. Furthermore, a specific extension implementation can only be registered
once for a given extension context and its parent contexts. Consequently, any attempt to
register a duplicate extension implementation will be ignored.

### [](#extensions-conditions)5.3. Conditional Test Execution ###

`[ExecutionCondition](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/ExecutionCondition.html)` defines the `Extension` API for programmatic, *conditional test
execution*.

An `ExecutionCondition` is *evaluated* for each container (e.g., a test class) to
determine if all the tests it contains should be executed based on the supplied`ExtensionContext`. Similarly, an `ExecutionCondition` is *evaluated* for each test to
determine if a given test method should be executed based on the supplied`ExtensionContext`.

When multiple `ExecutionCondition` extensions are registered, a container or test is
disabled as soon as one of the conditions returns *disabled*. Thus, there is no guarantee
that a condition is evaluated because another extension might have already caused a
container or test to be disabled. In other words, the evaluation works like the
short-circuiting boolean OR operator.

See the source code of `[DisabledCondition](https://github.com/junit-team/junit5/tree/r5.10.2/junit-jupiter-engine/src/main/java/org/junit/jupiter/engine/extension/DisabledCondition.java)` and `[@Disabled](../api/org.junit.jupiter.api/org/junit/jupiter/api/Disabled.html)` for concrete examples.

#### [](#extensions-conditions-deactivation)5.3.1. Deactivating Conditions ####

Sometimes it can be useful to run a test suite *without* certain conditions being active.
For example, you may wish to run tests even if they are annotated with `@Disabled` in
order to see if they are still *broken*. To do this, provide a pattern for the`junit.jupiter.conditions.deactivate` *configuration parameter* to specify which
conditions should be deactivated (i.e., not evaluated) for the current test run. The
pattern can be supplied as a JVM system property, as a *configuration parameter* in the`LauncherDiscoveryRequest` that is passed to the `Launcher`, or via the JUnit Platform
configuration file (see [Configuration Parameters](#running-tests-config-params) for details).

For example, to deactivate JUnit’s `@Disabled` condition, you can start your JVM with the
following system property.

`-Djunit.jupiter.conditions.deactivate=org.junit.*DisabledCondition`

##### [](#extensions-conditions-deactivation-patterns)Pattern Matching Syntax #####

Refer to [Pattern Matching Syntax](#running-tests-config-params-deactivation-pattern) for details.

### [](#extensions-test-instance-pre-construct-callback)5.4. Test Instance Pre-construct Callback ###

`[TestInstancePreConstructCallback](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/TestInstancePreConstructCallback.html)` defines the API for `Extensions` that wish to be invoked*prior* to test instances being constructed (by a constructor call or via`[TestInstanceFactory](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/TestInstanceFactory.html)`).

This extension provides a symmetric call to `[TestInstancePreDestroyCallback](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/TestInstancePreDestroyCallback.html)` and is useful
in combination with other extensions to prepare constructor parameters or keeping track of test
instances and their lifecycle.

### [](#extensions-test-instance-factories)5.5. Test Instance Factories ###

`[TestInstanceFactory](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/TestInstanceFactory.html)` defines the API for `Extensions` that wish to *create* test class
instances.

Common use cases include acquiring the test instance from a dependency injection
framework or invoking a static factory method to create the test class instance.

If no `TestInstanceFactory` is registered, the framework will invoke the *sole*constructor for the test class to instantiate it, potentially resolving constructor
arguments via registered `ParameterResolver` extensions.

Extensions that implement `TestInstanceFactory` can be registered on test interfaces,
top-level test classes, or `@Nested` test classes.

|   |Registering multiple extensions that implement `TestInstanceFactory` for any single class<br/>will result in an exception being thrown for all tests in that class, in any subclass,<br/>and in any nested class. Note that any `TestInstanceFactory` registered in a superclass<br/>or *enclosing* class (i.e., in the case of a `@Nested` test class) is *inherited*. It is<br/>the user’s responsibility to ensure that only a single `TestInstanceFactory` is<br/>registered for any specific test class.|
|---|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

### [](#extensions-test-instance-post-processing)5.6. Test Instance Post-processing ###

`[TestInstancePostProcessor](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/TestInstancePostProcessor.html)` defines the API for `Extensions` that wish to *post
process* test instances.

Common use cases include injecting dependencies into the test instance, invoking custom
initialization methods on the test instance, etc.

For a concrete example, consult the source code for the `[MockitoExtension](https://github.com/mockito/mockito/blob/release/2.x/subprojects/junit-jupiter/src/main/java/org/mockito/junit/jupiter/MockitoExtension.java)` and the`[SpringExtension](https://github.com/spring-projects/spring-framework/tree/HEAD/spring-test/src/main/java/org/springframework/test/context/junit/jupiter/SpringExtension.java)`.

### [](#extensions-test-instance-pre-destroy-callback)5.7. Test Instance Pre-destroy Callback ###

`[TestInstancePreDestroyCallback](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/TestInstancePreDestroyCallback.html)` defines the API for `Extensions` that wish to process
test instances *after* they have been used in tests and *before* they are destroyed.

Common use cases include cleaning dependencies that have been injected into the
test instance, invoking custom de-initialization methods on the test instance, etc.

### [](#extensions-parameter-resolution)5.8. Parameter Resolution ###

`[ParameterResolver](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/ParameterResolver.html)` defines the `Extension` API for dynamically resolving parameters at
runtime.

If a *test class* constructor, *test method*, or *lifecycle method* (see[Definitions](#writing-tests-definitions)) declares a parameter, the parameter must be *resolved* at
runtime by a `ParameterResolver`. A `ParameterResolver` can either be built-in (see`[TestInfoParameterResolver](https://github.com/junit-team/junit5/tree/r5.10.2/junit-jupiter-engine/src/main/java/org/junit/jupiter/engine/extension/TestInfoParameterResolver.java)`) or [registered by the user](#extensions-registration).
Generally speaking, parameters may be resolved by *name*, *type*, *annotation*, or any
combination thereof.

If you wish to implement a custom `[ParameterResolver](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/ParameterResolver.html)` that resolves parameters based
solely on the type of the parameter, you may find it convenient to extend the`[TypeBasedParameterResolver](https://github.com/junit-team/junit5/tree/r5.10.2/junit-jupiter-api/src/main/java/org/junit/jupiter/api/extension/support/TypeBasedParameterResolver.java)` which serves as a generic adapter for such use cases.

For concrete examples, consult the source code for `[CustomTypeParameterResolver](https://github.com/junit-team/junit5/tree/r5.10.2/junit-jupiter-engine/src/test/java/org/junit/jupiter/engine/execution/injection/sample/CustomTypeParameterResolver.java)`,`[CustomAnnotationParameterResolver](https://github.com/junit-team/junit5/tree/r5.10.2/junit-jupiter-engine/src/test/java/org/junit/jupiter/engine/execution/injection/sample/CustomAnnotationParameterResolver.java)`, and `[MapOfListsTypeBasedParameterResolver](https://github.com/junit-team/junit5/tree/r5.10.2/junit-jupiter-engine/src/test/java/org/junit/jupiter/engine/execution/injection/sample/MapOfListsTypeBasedParameterResolver.java)`.

|   |Due to a bug in the byte code generated by `javac` on JDK versions prior to JDK 9,<br/>looking up annotations on parameters directly via the core `java.lang.reflect.Parameter`API will always fail for *inner class* constructors (e.g., a constructor in a `@Nested`test class).<br/><br/>The `[ParameterContext](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/ParameterContext.html)` API supplied to `ParameterResolver` implementations therefore<br/>includes the following convenience methods for correctly looking up annotations on<br/>parameters. Extension authors are strongly encouraged to use these methods instead of<br/>those provided in `java.lang.reflect.Parameter` in order to avoid this bug in the JDK.<br/><br/>* `boolean isAnnotated(Class<? extends Annotation> annotationType)`<br/><br/>* `Optional<A> findAnnotation(Class<A> annotationType)`<br/><br/>* `List<A> findRepeatableAnnotations(Class<A> annotationType)`|
|---|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

|   |Other extensions can also leverage registered `ParameterResolvers` for method and<br/>constructor invocations, using the `[ExecutableInvoker](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/ExecutableInvoker.html)` available via the`getExecutableInvoker()` method in the `ExtensionContext`.|
|---|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

### [](#extensions-test-result-processing)5.9. Test Result Processing ###

`[TestWatcher](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/TestWatcher.html)` defines the API for extensions that wish to process the results of *test
method* executions. Specifically, a `TestWatcher` will be invoked with contextual
information for the following events.

* `testDisabled`: invoked after a disabled *test method* has been skipped

* `testSuccessful`: invoked after a *test method* has completed successfully

* `testAborted`: invoked after a *test method* has been aborted

* `testFailed`: invoked after a *test method* has failed

|   |In contrast to the definition of "test method" presented in[Definitions](#writing-tests-definitions), in this context *test method* refers to any `@Test` method<br/>or `@TestTemplate` method (for example, a `@RepeatedTest` or `@ParameterizedTest`).|
|---|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

Extensions implementing this interface can be registered at the class level, instance
level, or method level. When registered at the class level, a `TestWatcher` will be
invoked for any contained *test method* including those in `@Nested` classes. When
registered at the method level, a `TestWatcher` will only be invoked for the *test method*for which it was registered.

|   |If a `TestWatcher` is registered via a non-static (instance) field – for example, using`@RegisterExtension` – and the test class is configured with`@TestInstance(Lifecycle.PER_METHOD)` semantics (which is the default lifecycle mode), the`TestWatcher` will **not** be invoked with events for `@TestTemplate` methods (for<br/>example, `@RepeatedTest` or `@ParameterizedTest`).<br/><br/>To ensure that a `TestWatcher` is invoked for all *test methods* in a given class, it is<br/>therefore recommended that the `TestWatcher` be registered at the class level with`@ExtendWith` or via a `static` field with `@RegisterExtension` or `@ExtendWith`.|
|---|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

If there is a failure at the class level — for example, an exception thrown by a`@BeforeAll` method — no test results will be reported. Similarly, if the test class is
disabled via an `ExecutionCondition` — for example, `@Disabled` — no test results will be
reported.

In contrast to other Extension APIs, a `TestWatcher` is not permitted to adversely
influence the execution of tests. Consequently, any exception thrown by a method in the`TestWatcher` API will be logged at `WARNING` level and will not be allowed to propagate
or fail test execution.

|   |Any instances of `ExtensionContext.Store.CloseableResource` stored in the `Store` of the<br/>provided `[ExtensionContext](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/ExtensionContext.html)` will be closed *before* methods in the `TestWatcher` API are<br/>invoked (see [Keeping State in Extensions](#extensions-keeping-state)). You can use the parent context’s `Store` to<br/>work with such resources.|
|---|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

### [](#extensions-lifecycle-callbacks)5.10. Test Lifecycle Callbacks ###

The following interfaces define the APIs for extending tests at various points in the
test execution lifecycle. Consult the following sections for examples and the Javadoc for
each of these interfaces in the `[org.junit.jupiter.api.extension](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/package-summary.html)` package for further details.

* `[BeforeAllCallback](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/BeforeAllCallback.html)`

  * `[BeforeEachCallback](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/BeforeEachCallback.html)`

    * `[BeforeTestExecutionCallback](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/BeforeTestExecutionCallback.html)`

    * `[AfterTestExecutionCallback](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/AfterTestExecutionCallback.html)`

  * `[AfterEachCallback](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/AfterEachCallback.html)`

* `[AfterAllCallback](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/AfterAllCallback.html)`

|   |Implementing Multiple Extension APIs<br/><br/>Extension developers may choose to implement any number of these interfaces<br/>within a single extension. Consult the source code of the `[SpringExtension](https://github.com/spring-projects/spring-framework/tree/HEAD/spring-test/src/main/java/org/springframework/test/context/junit/jupiter/SpringExtension.java)` for a<br/>concrete example.|
|---|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

#### [](#extensions-lifecycle-callbacks-before-after-execution)5.10.1. Before and After Test Execution Callbacks ####

`[BeforeTestExecutionCallback](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/BeforeTestExecutionCallback.html)` and `[AfterTestExecutionCallback](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/AfterTestExecutionCallback.html)` define the APIs for`Extensions` that wish to add behavior that will be executed *immediately before* and*immediately after* a test method is executed, respectively. As such, these callbacks are
well suited for timing, tracing, and similar use cases. If you need to implement
callbacks that are invoked *around* `@BeforeEach` and `@AfterEach` methods, implement`BeforeEachCallback` and `AfterEachCallback` instead.

The following example shows how to use these callbacks to calculate and log the execution
time of a test method. `TimingExtension` implements both `BeforeTestExecutionCallback`and `AfterTestExecutionCallback` in order to time and log the test execution.

An extension that times and logs the execution of test methods

```
import java.lang.reflect.Method;
import java.util.logging.Logger;

import org.junit.jupiter.api.extension.AfterTestExecutionCallback;
import org.junit.jupiter.api.extension.BeforeTestExecutionCallback;
import org.junit.jupiter.api.extension.ExtensionContext;
import org.junit.jupiter.api.extension.ExtensionContext.Namespace;
import org.junit.jupiter.api.extension.ExtensionContext.Store;

public class TimingExtension implements BeforeTestExecutionCallback, AfterTestExecutionCallback {

    private static final Logger logger = Logger.getLogger(TimingExtension.class.getName());

    private static final String START_TIME = "start time";

    @Override
    public void beforeTestExecution(ExtensionContext context) throws Exception {
        getStore(context).put(START_TIME, System.currentTimeMillis());
    }

    @Override
    public void afterTestExecution(ExtensionContext context) throws Exception {
        Method testMethod = context.getRequiredTestMethod();
        long startTime = getStore(context).remove(START_TIME, long.class);
        long duration = System.currentTimeMillis() - startTime;

        logger.info(() ->
            String.format("Method [%s] took %s ms.", testMethod.getName(), duration));
    }

    private Store getStore(ExtensionContext context) {
        return context.getStore(Namespace.create(getClass(), context.getRequiredTestMethod()));
    }

}
```

Since the `TimingExtensionTests` class registers the `TimingExtension` via `@ExtendWith`,
its tests will have this timing applied when they execute.

A test class that uses the example TimingExtension

```
@ExtendWith(TimingExtension.class)
class TimingExtensionTests {

    @Test
    void sleep20ms() throws Exception {
        Thread.sleep(20);
    }

    @Test
    void sleep50ms() throws Exception {
        Thread.sleep(50);
    }

}
```

The following is an example of the logging produced when `TimingExtensionTests` is run.

```
INFO: Method [sleep20ms] took 24 ms.
INFO: Method [sleep50ms] took 53 ms.
```

### [](#extensions-exception-handling)5.11. Exception Handling ###

Exceptions thrown during the test execution may be intercepted and handled accordingly
before propagating further, so that certain actions like error logging or resource releasing
may be defined in specialized `Extensions`. JUnit Jupiter offers API for `Extensions` that
wish to handle exceptions thrown during `@Test` methods via `[TestExecutionExceptionHandler](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/TestExecutionExceptionHandler.html)`and for those thrown during one of test lifecycle methods (`@BeforeAll`, `@BeforeEach`,`@AfterEach` and `@AfterAll`) via `[LifecycleMethodExecutionExceptionHandler](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/LifecycleMethodExecutionExceptionHandler.html)`.

The following example shows an extension which will swallow all instances of `IOException`but rethrow any other type of exception.

An exception handling extension that filters IOExceptions in test execution

```
public class IgnoreIOExceptionExtension implements TestExecutionExceptionHandler {

    @Override
    public void handleTestExecutionException(ExtensionContext context, Throwable throwable)
            throws Throwable {

        if (throwable instanceof IOException) {
            return;
        }
        throw throwable;
    }
}
```

Another example shows how to record the state of an application under test exactly at
the point of unexpected exception being thrown during setup and cleanup. Note that unlike
relying on lifecycle callbacks, which may or may not be executed depending on the test
status, this solution guarantees execution immediately after failing `@BeforeAll`,`@BeforeEach`, `@AfterEach` or `@AfterAll`.

An exception handling extension that records application state on error

```
class RecordStateOnErrorExtension implements LifecycleMethodExecutionExceptionHandler {

    @Override
    public void handleBeforeAllMethodExecutionException(ExtensionContext context, Throwable ex)
            throws Throwable {
        memoryDumpForFurtherInvestigation("Failure recorded during class setup");
        throw ex;
    }

    @Override
    public void handleBeforeEachMethodExecutionException(ExtensionContext context, Throwable ex)
            throws Throwable {
        memoryDumpForFurtherInvestigation("Failure recorded during test setup");
        throw ex;
    }

    @Override
    public void handleAfterEachMethodExecutionException(ExtensionContext context, Throwable ex)
            throws Throwable {
        memoryDumpForFurtherInvestigation("Failure recorded during test cleanup");
        throw ex;
    }

    @Override
    public void handleAfterAllMethodExecutionException(ExtensionContext context, Throwable ex)
            throws Throwable {
        memoryDumpForFurtherInvestigation("Failure recorded during class cleanup");
        throw ex;
    }
}
```

Multiple execution exception handlers may be invoked for the same lifecycle method in
order of declaration. If one of the handlers swallows the handled exception, subsequent
ones will not be executed, and no failure will be propagated to JUnit engine, as if the
exception was never thrown. Handlers may also choose to rethrow the exception or throw
a different one, potentially wrapping the original.

Extensions implementing `[LifecycleMethodExecutionExceptionHandler](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/LifecycleMethodExecutionExceptionHandler.html)` that wish to handle
exceptions thrown during `@BeforeAll` or `@AfterAll` need to be registered on a class level,
while handlers for `BeforeEach` and `AfterEach` may be also registered for individual
test methods.

Registering multiple exception handling extensions

```
// Register handlers for @Test, @BeforeEach, @AfterEach as well as @BeforeAll and @AfterAll
@ExtendWith(ThirdExecutedHandler.class)
class MultipleHandlersTestCase {

    // Register handlers for @Test, @BeforeEach, @AfterEach only
    @ExtendWith(SecondExecutedHandler.class)
    @ExtendWith(FirstExecutedHandler.class)
    @Test
    void testMethod() {
    }

}
```

### [](#extensions-intercepting-invocations)5.12. Intercepting Invocations ###

`[InvocationInterceptor](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/InvocationInterceptor.html)` defines the API for `Extensions` that wish to intercept calls to
test code.

The following example shows an extension that executes all test methods in Swing’s Event
Dispatch Thread.

An extension that executes tests in a user-defined thread

```
public class SwingEdtInterceptor implements InvocationInterceptor {

    @Override
    public void interceptTestMethod(Invocation<Void> invocation,
            ReflectiveInvocationContext<Method> invocationContext,
            ExtensionContext extensionContext) throws Throwable {

        AtomicReference<Throwable> throwable = new AtomicReference<>();

        SwingUtilities.invokeAndWait(() -> {
            try {
                invocation.proceed();
            }
            catch (Throwable t) {
                throwable.set(t);
            }
        });
        Throwable t = throwable.get();
        if (t != null) {
            throw t;
        }
    }
}
```

### [](#extensions-test-templates)5.13. Providing Invocation Contexts for Test Templates ###

A `[@TestTemplate](../api/org.junit.jupiter.api/org/junit/jupiter/api/TestTemplate.html)` method can only be executed when at least one`[TestTemplateInvocationContextProvider](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/TestTemplateInvocationContextProvider.html)` is registered. Each such provider is responsible
for providing a `Stream` of `[TestTemplateInvocationContext](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/TestTemplateInvocationContext.html)` instances. Each context may
specify a custom display name and a list of additional extensions that will only be used
for the next invocation of the `[@TestTemplate](../api/org.junit.jupiter.api/org/junit/jupiter/api/TestTemplate.html)` method.

The following example shows how to write a test template as well as how to register and
implement a `[TestTemplateInvocationContextProvider](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/TestTemplateInvocationContextProvider.html)`.

A test template with accompanying extension

```
final List<String> fruits = Arrays.asList("apple", "banana", "lemon");

@TestTemplate
@ExtendWith(MyTestTemplateInvocationContextProvider.class)
void testTemplate(String fruit) {
    assertTrue(fruits.contains(fruit));
}

public class MyTestTemplateInvocationContextProvider
        implements TestTemplateInvocationContextProvider {

    @Override
    public boolean supportsTestTemplate(ExtensionContext context) {
        return true;
    }

    @Override
    public Stream<TestTemplateInvocationContext> provideTestTemplateInvocationContexts(
            ExtensionContext context) {

        return Stream.of(invocationContext("apple"), invocationContext("banana"));
    }

    private TestTemplateInvocationContext invocationContext(String parameter) {
        return new TestTemplateInvocationContext() {
            @Override
            public String getDisplayName(int invocationIndex) {
                return parameter;
            }

            @Override
            public List<Extension> getAdditionalExtensions() {
                return Collections.singletonList(new ParameterResolver() {
                    @Override
                    public boolean supportsParameter(ParameterContext parameterContext,
                            ExtensionContext extensionContext) {
                        return parameterContext.getParameter().getType().equals(String.class);
                    }

                    @Override
                    public Object resolveParameter(ParameterContext parameterContext,
                            ExtensionContext extensionContext) {
                        return parameter;
                    }
                });
            }
        };
    }
}
```

In this example, the test template will be invoked twice. The display names of the
invocations will be `apple` and `banana` as specified by the invocation context. Each
invocation registers a custom `[ParameterResolver](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/ParameterResolver.html)` which is used to resolve the method
parameter. The output when using the `ConsoleLauncher` is as follows.

```
└─ testTemplate(String) ✔
   ├─ apple ✔
   └─ banana ✔
```

The `[TestTemplateInvocationContextProvider](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/TestTemplateInvocationContextProvider.html)` extension API is primarily intended for
implementing different kinds of tests that rely on repetitive invocation of a test-like
method albeit in different contexts — for example, with different parameters, by preparing
the test class instance differently, or multiple times without modifying the context.
Please refer to the implementations of [Repeated Tests](#writing-tests-repeated-tests) or[Parameterized Tests](#writing-tests-parameterized-tests) which use this extension point to provide their
functionality.

### [](#extensions-keeping-state)5.14. Keeping State in Extensions ###

Usually, an extension is instantiated only once. So the question becomes relevant: How do
you keep the state from one invocation of an extension to the next? The`ExtensionContext` API provides a `Store` exactly for this purpose. Extensions may put
values into a store for later retrieval. See the`[TimingExtension](#extensions-lifecycle-callbacks-timing-extension)` for an example of
using the `Store` with a method-level scope. It is important to remember that values
stored in an `ExtensionContext` during test execution will not be available in the
surrounding `ExtensionContext`. Since `ExtensionContexts` may be nested, the scope of
inner contexts may also be limited. Consult the corresponding Javadoc for details on the
methods available for storing and retrieving values via the `[Store](../api/org.junit.jupiter.api/org/junit/jupiter/api/extension/ExtensionContext.Store.html)`.

|   |`ExtensionContext.Store.CloseableResource`<br/><br/>An extension context store is bound to its extension context lifecycle. When an<br/>extension context lifecycle ends it closes its associated store. All stored values<br/>that are instances of `CloseableResource` are notified by an invocation of their `close()`method in the inverse order they were added in.|
|---|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

### [](#extensions-supported-utilities)5.15. Supported Utilities in Extensions ###

The `junit-platform-commons` artifact exposes a package named`[org.junit.platform.commons.support](../api/org.junit.platform.commons/org/junit/platform/commons/support/package-summary.html)` that contains *maintained* utility methods for working
with annotations, classes, reflection, and classpath scanning tasks. `TestEngine` and`Extension` authors are encouraged to use these supported methods in order to align with
the behavior of the JUnit Platform.

#### [](#extensions-supported-utilities-annotations)5.15.1. Annotation Support ####

`AnnotationSupport` provides static utility methods that operate on annotated elements
(e.g., packages, annotations, classes, interfaces, constructors, methods, and fields).
These include methods to check whether an element is annotated or meta-annotated with a
particular annotation, to search for specific annotations, and to find annotated methods
and fields in a class or interface. Some of these methods search on implemented
interfaces and within class hierarchies to find annotations. Consult the Javadoc for`[AnnotationSupport](../api/org.junit.platform.commons/org/junit/platform/commons/support/AnnotationSupport.html)` for further details.

#### [](#extensions-supported-utilities-classes)5.15.2. Class Support ####

`ClassSupport` provides static utility methods for working with classes (i.e., instances
of `java.lang.Class`). Consult the Javadoc for `[ClassSupport](../api/org.junit.platform.commons/org/junit/platform/commons/support/ClassSupport.html)` for further details.

#### [](#extensions-supported-utilities-reflection)5.15.3. Reflection Support ####

`ReflectionSupport` provides static utility methods that augment the standard JDK
reflection and class-loading mechanisms. These include methods to scan the classpath in
search of classes matching specified predicates, to load and create new instances of a
class, and to find and invoke methods. Some of these methods traverse class hierarchies
to locate matching methods. Consult the Javadoc for `[ReflectionSupport](../api/org.junit.platform.commons/org/junit/platform/commons/support/ReflectionSupport.html)` for further
details.

#### [](#extensions-supported-utilities-modifier)5.15.4. Modifier Support ####

`ModifierSupport` provides static utility methods for working with member and class
modifiers — for example, to determine if a member is declared as `public`, `private`,`abstract`, `static`, etc. Consult the Javadoc for `[ModifierSupport](../api/org.junit.platform.commons/org/junit/platform/commons/support/ModifierSupport.html)` for further
details.

### [](#extensions-execution-order)5.16. Relative Execution Order of User Code and Extensions ###

When executing a test class that contains one or more test methods, a number of extension
callbacks are called in addition to the user-supplied test and lifecycle methods.

|   |See also: [Test Execution Order](#writing-tests-test-execution-order)|
|---|---------------------------------------------------------------------|

#### [](#extensions-execution-order-overview)5.16.1. User and Extension Code ####

The following diagram illustrates the relative order of user-supplied code and extension
code. User-supplied test and lifecycle methods are shown in orange, with callback code
implemented by extensions shown in blue. The grey box denotes the execution of a single
test method and will be repeated for every test method in the test class.

![extensions lifecycle](images/extensions_lifecycle.png)

User code and extension code

The following table further explains the sixteen steps in the[User code and extension code](#extensions-execution-order-diagram) diagram.

|Step|                                                       Interface/Annotation                                                       |                                           Description                                           |
|----|----------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------|
| 1  |                                  interface `org.junit.jupiter.api.extension.BeforeAllCallback`                                   |             extension code executed before all tests of the container are executed              |
| 2  |                                           annotation `org.junit.jupiter.api.BeforeAll`                                           |                user code executed before all tests of the container are executed                |
| 3  |interface `org.junit.jupiter.api.extension.LifecycleMethodExecutionExceptionHandler<br/>#handleBeforeAllMethodExecutionException` |             extension code for handling exceptions thrown from `@BeforeAll` methods             |
| 4  |                                  interface `org.junit.jupiter.api.extension.BeforeEachCallback`                                  |                      extension code executed before each test is executed                       |
| 5  |                                          annotation `org.junit.jupiter.api.BeforeEach`                                           |                         user code executed before each test is executed                         |
| 6  |interface `org.junit.jupiter.api.extension.LifecycleMethodExecutionExceptionHandler<br/>#handleBeforeEachMethodExecutionException`|            extension code for handling exceptions thrown from `@BeforeEach` methods             |
| 7  |                             interface `org.junit.jupiter.api.extension.BeforeTestExecutionCallback`                              |                  extension code executed immediately before a test is executed                  |
| 8  |                                             annotation `org.junit.jupiter.api.Test`                                              |                               user code of the actual test method                               |
| 9  |                            interface `org.junit.jupiter.api.extension.TestExecutionExceptionHandler`                             |                   extension code for handling exceptions thrown during a test                   |
| 10 |                              interface `org.junit.jupiter.api.extension.AfterTestExecutionCallback`                              |extension code executed immediately after test execution and its corresponding exception handlers|
| 11 |                                           annotation `org.junit.jupiter.api.AfterEach`                                           |                         user code executed after each test is executed                          |
| 12 |interface `org.junit.jupiter.api.extension.LifecycleMethodExecutionExceptionHandler<br/>#handleAfterEachMethodExecutionException` |             extension code for handling exceptions thrown from `@AfterEach` methods             |
| 13 |                                  interface `org.junit.jupiter.api.extension.AfterEachCallback`                                   |                       extension code executed after each test is executed                       |
| 14 |                                           annotation `org.junit.jupiter.api.AfterAll`                                            |                user code executed after all tests of the container are executed                 |
| 15 | interface `org.junit.jupiter.api.extension.LifecycleMethodExecutionExceptionHandler<br/>#handleAfterAllMethodExecutionException` |             extension code for handling exceptions thrown from `@AfterAll` methods              |
| 16 |                                   interface `org.junit.jupiter.api.extension.AfterAllCallback`                                   |              extension code executed after all tests of the container are executed              |

In the simplest case only the actual test method will be executed (step 8); all other
steps are optional depending on the presence of user code or extension support for the
corresponding lifecycle callback. For further details on the various lifecycle callbacks
please consult the respective Javadoc for each annotation and extension.

All invocations of user code methods in the above table can additionally be intercepted
by implementing [`InvocationInterceptor`](#extensions-intercepting-invocations).

#### [](#extensions-execution-order-wrapping-behavior)5.16.2. Wrapping Behavior of Callbacks ####

JUnit Jupiter always guarantees *wrapping* behavior for multiple registered extensions
that implement lifecycle callbacks such as `BeforeAllCallback`, `AfterAllCallback`,`BeforeEachCallback`, `AfterEachCallback`, `BeforeTestExecutionCallback`, and`AfterTestExecutionCallback`.

That means that, given two extensions `Extension1` and `Extension2` with `Extension1`registered before `Extension2`, any "before" callbacks implemented by `Extension1` are
guaranteed to execute **before** any "before" callbacks implemented by `Extension2`.
Similarly, given the two same two extensions registered in the same order, any "after"
callbacks implemented by `Extension1` are guaranteed to execute **after** any "after"
callbacks implemented by `Extension2`. `Extension1` is therefore said to *wrap*`Extension2`.

JUnit Jupiter also guarantees *wrapping* behavior within class and interface hierarchies
for user-supplied *lifecycle methods* (see [Definitions](#writing-tests-definitions)).

* `@BeforeAll` methods are inherited from superclasses as long as they are not *hidden*,*overridden*, or *superseded* (i.e., replaced based on signature only, irrespective of
  Java’s visibility rules). Furthermore, `@BeforeAll` methods from superclasses will be
  executed **before** `@BeforeAll` methods in subclasses.

  * Similarly, `@BeforeAll` methods declared in an interface are inherited as long as they
    are not *hidden* or *overridden*, and `@BeforeAll` methods from an interface will be
    executed **before** `@BeforeAll` methods in the class that implements the interface.

* `@AfterAll` methods are inherited from superclasses as long as they are not *hidden*,*overridden*, or *superseded* (i.e., replaced based on signature only, irrespective of
  Java’s visibility rules). Furthermore, `@AfterAll` methods from superclasses will be
  executed **after** `@AfterAll` methods in subclasses.

  * Similarly, `@AfterAll` methods declared in an interface are inherited as long as they
    are not *hidden* or *overridden*, and `@AfterAll` methods from an interface will be
    executed **after** `@AfterAll` methods in the class that implements the interface.

* `@BeforeEach` methods are inherited from superclasses as long as they are not*overridden* or *superseded* (i.e., replaced based on signature only, irrespective of
  Java’s visibility rules). Furthermore, `@BeforeEach` methods from superclasses will be
  executed **before** `@BeforeEach` methods in subclasses.

  * Similarly, `@BeforeEach` methods declared as interface default methods are inherited as
    long as they are not *overridden*, and `@BeforeEach` default methods will be executed**before** `@BeforeEach` methods in the class that implements the interface.

* `@AfterEach` methods are inherited from superclasses as long as they are not*overridden* or *superseded* (i.e., replaced based on signature only, irrespective of
  Java’s visibility rules). Furthermore, `@AfterEach` methods from superclasses will be
  executed **after** `@AfterEach` methods in subclasses.

  * Similarly, `@AfterEach` methods declared as interface default methods are inherited as
    long as they are not *overridden*, and `@AfterEach` default methods will be executed**after** `@AfterEach` methods in the class that implements the interface.

The following examples demonstrate this behavior. Please note that the examples do not
actually do anything realistic. Instead, they mimic common scenarios for testing
interactions with the database. All methods imported statically from the `Logger` class
log contextual information in order to help us better understand the execution order of
user-supplied callback methods and callback methods in extensions.

Extension1

```
import static example.callbacks.Logger.afterEachCallback;
import static example.callbacks.Logger.beforeEachCallback;

import org.junit.jupiter.api.extension.AfterEachCallback;
import org.junit.jupiter.api.extension.BeforeEachCallback;
import org.junit.jupiter.api.extension.ExtensionContext;

public class Extension1 implements BeforeEachCallback, AfterEachCallback {

    @Override
    public void beforeEach(ExtensionContext context) {
        beforeEachCallback(this);
    }

    @Override
    public void afterEach(ExtensionContext context) {
        afterEachCallback(this);
    }

}
```

Extension2

```
import static example.callbacks.Logger.afterEachCallback;
import static example.callbacks.Logger.beforeEachCallback;

import org.junit.jupiter.api.extension.AfterEachCallback;
import org.junit.jupiter.api.extension.BeforeEachCallback;
import org.junit.jupiter.api.extension.ExtensionContext;

public class Extension2 implements BeforeEachCallback, AfterEachCallback {

    @Override
    public void beforeEach(ExtensionContext context) {
        beforeEachCallback(this);
    }

    @Override
    public void afterEach(ExtensionContext context) {
        afterEachCallback(this);
    }

}
```

AbstractDatabaseTests

```
import static example.callbacks.Logger.afterAllMethod;
import static example.callbacks.Logger.afterEachMethod;
import static example.callbacks.Logger.beforeAllMethod;
import static example.callbacks.Logger.beforeEachMethod;

import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;

/**
 * Abstract base class for tests that use the database.
 */
abstract class AbstractDatabaseTests {

    @BeforeAll
    static void createDatabase() {
        beforeAllMethod(AbstractDatabaseTests.class.getSimpleName() + ".createDatabase()");
    }

    @BeforeEach
    void connectToDatabase() {
        beforeEachMethod(AbstractDatabaseTests.class.getSimpleName() + ".connectToDatabase()");
    }

    @AfterEach
    void disconnectFromDatabase() {
        afterEachMethod(AbstractDatabaseTests.class.getSimpleName() + ".disconnectFromDatabase()");
    }

    @AfterAll
    static void destroyDatabase() {
        afterAllMethod(AbstractDatabaseTests.class.getSimpleName() + ".destroyDatabase()");
    }

}
```

DatabaseTestsDemo

```
import static example.callbacks.Logger.afterEachMethod;
import static example.callbacks.Logger.beforeAllMethod;
import static example.callbacks.Logger.beforeEachMethod;
import static example.callbacks.Logger.testMethod;

import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

/**
 * Extension of {@link AbstractDatabaseTests} that inserts test data
 * into the database (after the database connection has been opened)
 * and deletes test data (before the database connection is closed).
 */
@ExtendWith({ Extension1.class, Extension2.class })
class DatabaseTestsDemo extends AbstractDatabaseTests {

    @BeforeAll
    static void beforeAll() {
        beforeAllMethod(DatabaseTestsDemo.class.getSimpleName() + ".beforeAll()");
    }

    @BeforeEach
    void insertTestDataIntoDatabase() {
        beforeEachMethod(getClass().getSimpleName() + ".insertTestDataIntoDatabase()");
    }

    @Test
    void testDatabaseFunctionality() {
        testMethod(getClass().getSimpleName() + ".testDatabaseFunctionality()");
    }

    @AfterEach
    void deleteTestDataFromDatabase() {
        afterEachMethod(getClass().getSimpleName() + ".deleteTestDataFromDatabase()");
    }

    @AfterAll
    static void afterAll() {
        beforeAllMethod(DatabaseTestsDemo.class.getSimpleName() + ".afterAll()");
    }

}
```

When the `DatabaseTestsDemo` test class is executed, the following is logged.

```
@BeforeAll AbstractDatabaseTests.createDatabase()
@BeforeAll DatabaseTestsDemo.beforeAll()
  Extension1.beforeEach()
  Extension2.beforeEach()
    @BeforeEach AbstractDatabaseTests.connectToDatabase()
    @BeforeEach DatabaseTestsDemo.insertTestDataIntoDatabase()
      @Test DatabaseTestsDemo.testDatabaseFunctionality()
    @AfterEach DatabaseTestsDemo.deleteTestDataFromDatabase()
    @AfterEach AbstractDatabaseTests.disconnectFromDatabase()
  Extension2.afterEach()
  Extension1.afterEach()
@BeforeAll DatabaseTestsDemo.afterAll()
@AfterAll AbstractDatabaseTests.destroyDatabase()
```

The following sequence diagram helps to shed further light on what actually goes on within
the `JupiterTestEngine` when the `DatabaseTestsDemo` test class is executed.

![extensions DatabaseTestsDemo](images/extensions_DatabaseTestsDemo.png)

DatabaseTestsDemo

JUnit Jupiter does **not** guarantee the execution order of multiple lifecycle methods
that are declared within a *single* test class or test interface. It may at times appear
that JUnit Jupiter invokes such methods in alphabetical order. However, that is not
precisely true. The ordering is analogous to the ordering for `@Test` methods within a
single test class.

|   |Lifecycle methods that are declared within a *single* test class or test interface will be<br/>ordered using an algorithm that is deterministic but intentionally non-obvious. This<br/>ensures that subsequent runs of a test suite execute lifecycle methods in the same order,<br/>thereby allowing for repeatable builds.|
|---|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

In addition, JUnit Jupiter does **not** support *wrapping* behavior for multiple lifecycle
methods declared within a single test class or test interface.

The following example demonstrates this behavior. Specifically, the lifecycle method
configuration is *broken* due to the order in which the locally declared lifecycle methods
are executed.

* Test data is inserted *before* the database connection has been opened, which results in
  a failure to connect to the database.

* The database connection is closed *before* deleting the test data, which results in a
  failure to connect to the database.

BrokenLifecycleMethodConfigDemo

```
import static example.callbacks.Logger.afterEachMethod;
import static example.callbacks.Logger.beforeEachMethod;
import static example.callbacks.Logger.testMethod;

import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

/**
 * Example of "broken" lifecycle method configuration.
 *
 * <p>Test data is inserted before the database connection has been opened.
 *
 * <p>Database connection is closed before deleting test data.
 */
@ExtendWith({ Extension1.class, Extension2.class })
class BrokenLifecycleMethodConfigDemo {

    @BeforeEach
    void connectToDatabase() {
        beforeEachMethod(getClass().getSimpleName() + ".connectToDatabase()");
    }

    @BeforeEach
    void insertTestDataIntoDatabase() {
        beforeEachMethod(getClass().getSimpleName() + ".insertTestDataIntoDatabase()");
    }

    @Test
    void testDatabaseFunctionality() {
        testMethod(getClass().getSimpleName() + ".testDatabaseFunctionality()");
    }

    @AfterEach
    void deleteTestDataFromDatabase() {
        afterEachMethod(getClass().getSimpleName() + ".deleteTestDataFromDatabase()");
    }

    @AfterEach
    void disconnectFromDatabase() {
        afterEachMethod(getClass().getSimpleName() + ".disconnectFromDatabase()");
    }

}
```

When the `BrokenLifecycleMethodConfigDemo` test class is executed, the following is logged.

```
Extension1.beforeEach()
Extension2.beforeEach()
  @BeforeEach BrokenLifecycleMethodConfigDemo.insertTestDataIntoDatabase()
  @BeforeEach BrokenLifecycleMethodConfigDemo.connectToDatabase()
    @Test BrokenLifecycleMethodConfigDemo.testDatabaseFunctionality()
  @AfterEach BrokenLifecycleMethodConfigDemo.disconnectFromDatabase()
  @AfterEach BrokenLifecycleMethodConfigDemo.deleteTestDataFromDatabase()
Extension2.afterEach()
Extension1.afterEach()
```

The following sequence diagram helps to shed further light on what actually goes on within
the `JupiterTestEngine` when the `BrokenLifecycleMethodConfigDemo` test class is executed.

![extensions BrokenLifecycleMethodConfigDemo](images/extensions_BrokenLifecycleMethodConfigDemo.png)

BrokenLifecycleMethodConfigDemo

|   |Due to the aforementioned behavior, the JUnit Team recommends that developers declare at<br/>most one of each type of *lifecycle method* (see [Definitions](#writing-tests-definitions)) per test<br/>class or test interface unless there are no dependencies between such lifecycle methods.|
|---|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

[](#advanced-topics)6. Advanced Topics
----------

### [](#junit-platform-reporting)6.1. JUnit Platform Reporting ###

The `junit-platform-reporting` artifact contains `[TestExecutionListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/TestExecutionListener.html)` implementations
that generate XML test reports in two flavors:[legacy](#junit-platform-reporting-legacy-xml) and[Open Test Reporting](#junit-platform-reporting-open-test-reporting).

|   |The module also contains other `TestExecutionListener` implementations that can be<br/>used to build custom reporting. See [Using Listeners and Interceptors](#running-tests-listeners) for details.|
|---|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

#### [](#junit-platform-reporting-legacy-xml)6.1.1. Legacy XML format ####

`[LegacyXmlReportGeneratingListener](../api/org.junit.platform.reporting/org/junit/platform/reporting/legacy/xml/LegacyXmlReportGeneratingListener.html)` generates a separate XML report for each root in the`[TestPlan](../api/org.junit.platform.launcher/org/junit/platform/launcher/TestPlan.html)`. Note that the generated XML format is compatible with the de facto standard
for JUnit 4 based test reports that was made popular by the Ant build system.

The `LegacyXmlReportGeneratingListener` is used by the [Console Launcher](#running-tests-console-launcher)as well.

#### [](#junit-platform-reporting-open-test-reporting)6.1.2. Open Test Reporting XML format ####

`[OpenTestReportGeneratingListener](../api/org.junit.platform.reporting/org/junit/platform/reporting/open/xml/OpenTestReportGeneratingListener.html)` writes an XML report for the entire execution in the
event-based format specified by [Open Test Reporting](https://github.com/ota4j-team/open-test-reporting) which supports all features of the
JUnit Platform such as hierarchical test structures, display names, tags, etc.

The listener is auto-registered and can be configured via the following[Configuration Parameters](#running-tests-config-params):

`junit.platform.reporting.open.xml.enabled=true|false`

Enable/disable writing the report.

`junit.platform.reporting.output.dir=<path>`

Configure the output directory for the reports. By default, `build` is used if a Gradle
build script is found, and `target` if a Maven POM is found; otherwise, the current
working directory is used.

If enabled, the listener creates an XML report file named`junit-platform-events-<random-id>.xml` per test run in the configured output directory.

|   |The [Open Test Reporting CLI Tool](https://github.com/ota4j-team/open-test-reporting#cli-tool-for-validation-and-format-conversion) can be used to convert from the event-based format to<br/>the hierarchical format which is more human-readable.|
|---|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

##### [](#gradle)Gradle #####

For Gradle, writing Open Test Reporting compatible XML reports can be enabled and
configured via system properties. The following samples configure its output directory to
be the same directory Gradle uses for its own XML reports. A `CommandLineArgumentProvider`is used to keep the tasks relocatable across different machines which is important when
using Gradle’s Build Cache.

Groovy DSL

```
dependencies {
    testRuntimeOnly("org.junit.platform:junit-platform-reporting:1.10.2")
}
tasks.withType(Test).configureEach {
    def outputDir = reports.junitXml.outputLocation
    jvmArgumentProviders << ({
        [
            "-Djunit.platform.reporting.open.xml.enabled=true",
            "-Djunit.platform.reporting.output.dir=${outputDir.get().asFile.absolutePath}"
        ]
    } as CommandLineArgumentProvider)
}
```

Kotlin DSL

```
dependencies {
    testRuntimeOnly("org.junit.platform:junit-platform-reporting:1.10.2")
}
tasks.withType<Test>().configureEach {
    val outputDir = reports.junitXml.outputLocation
    jvmArgumentProviders += CommandLineArgumentProvider {
        listOf(
            "-Djunit.platform.reporting.open.xml.enabled=true",
            "-Djunit.platform.reporting.output.dir=${outputDir.get().asFile.absolutePath}"
        )
    }
}
```

##### [](#maven)Maven #####

For Maven Surefire/Failsafe, you can enable Open Test Reporting output and configure the
resulting XML files to be written to the same directory Surefire/Failsafe uses for its own
XML reports as follows:

```
<project>
    <!-- ... -->
    <dependencies>
        <dependency>
            <groupId>org.junit.platform</groupId>
            <artifactId>junit-platform-reporting</artifactId>
            <version>1.10.2</version>
            <scope>test</scope>
        </dependency>
    </dependencies>
    <build>
        <plugins>
            <plugin>
                <artifactId>maven-surefire-plugin</artifactId>
                <version>3.1.2</version>
                <configuration>
                    <properties>
                        <configurationParameters>
                            junit.platform.reporting.open.xml.enabled = true
                            junit.platform.reporting.output.dir = target/surefire-reports
                        </configurationParameters>
                    </properties>
                </configuration>
            </plugin>
        </plugins>
    </build>
    <!-- ... -->
</project>
```

##### [](#console-launcher)Console Launcher #####

When using the [Console Launcher](#running-tests-console-launcher), you can enable Open Test Reporting
output by setting the configuration parameters via `--config`:

```
$ java -jar junit-platform-console-standalone-1.10.2.jar <OPTIONS> \
  --config=junit.platform.reporting.open.xml.enabled=true \
  --config=junit.platform.reporting.output.dir=reports
```

### [](#junit-platform-suite-engine)6.2. JUnit Platform Suite Engine ###

The JUnit Platform supports the declarative definition and execution of suites of tests
from *any* test engine using the JUnit Platform.

#### [](#junit-platform-suite-engine-setup)6.2.1. Setup ####

In addition to the `junit-platform-suite-api` and `junit-platform-suite-engine` artifacts,
you need *at least one* other test engine and its dependencies on the classpath. See[Dependency Metadata](#dependency-metadata) for details regarding group IDs, artifact IDs, and versions.

##### [](#junit-platform-suite-engine-setup-required-dependencies)Required Dependencies #####

* `junit-platform-suite-api` in *test* scope: artifact containing annotations needed to
  configure a test suite

* `junit-platform-suite-engine` in *test runtime* scope: implementation of the`TestEngine` API for declarative test suites

|   |Both of the required dependencies are aggregated in the `junit-platform-suite`artifact which can be declared in *test* scope instead of declaring explicit dependencies<br/>on `junit-platform-suite-api` and `junit-platform-suite-engine`.|
|---|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

##### [](#junit-platform-suite-engine-setup-transitive-dependencies)Transitive Dependencies #####

* `junit-platform-suite-commons` in *test* scope

* `junit-platform-launcher` in *test* scope

* `junit-platform-engine` in *test* scope

* `junit-platform-commons` in *test* scope

* `opentest4j` in *test* scope

#### [](#junit-platform-suite-engine-example)6.2.2. @Suite Example ####

By annotating a class with `@Suite` it is marked as a test suite on the JUnit Platform.
As seen in the following example, selector and filter annotations can then be used to
control the contents of the suite.

```
import org.junit.platform.suite.api.IncludeClassNamePatterns;
import org.junit.platform.suite.api.SelectPackages;
import org.junit.platform.suite.api.Suite;
import org.junit.platform.suite.api.SuiteDisplayName;

@Suite
@SuiteDisplayName("JUnit Platform Suite Demo")
@SelectPackages("example")
@IncludeClassNamePatterns(".*Tests")
class SuiteDemo {
}
```

|   |Additional Configuration Options<br/><br/>There are numerous configuration options for discovering and filtering tests in a<br/>test suite. Please consult the Javadoc of the `[org.junit.platform.suite.api](../api/org.junit.platform.suite.api/org/junit/platform/suite/api/package-summary.html)` package for a full<br/>list of supported annotations and further details.|
|---|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

### [](#testkit)6.3. JUnit Platform Test Kit ###

The `junit-platform-testkit` artifact provides support for executing a test plan on the
JUnit Platform and then verifying the expected results. As of JUnit Platform 1.4, this
support is limited to the execution of a single `TestEngine` (see [Engine Test Kit](#testkit-engine)).

#### [](#testkit-engine)6.3.1. Engine Test Kit ####

The `[org.junit.platform.testkit.engine](../api/org.junit.platform.testkit/org/junit/platform/testkit/engine/package-summary.html)` package provides support for executing a `[TestPlan](../api/org.junit.platform.launcher/org/junit/platform/launcher/TestPlan.html)` for a
given `[TestEngine](../api/org.junit.platform.engine/org/junit/platform/engine/TestEngine.html)` running on the JUnit Platform and then accessing the results via a
fluent API to verify the expected results. The key entry point into this API is the`[EngineTestKit](../api/org.junit.platform.testkit/org/junit/platform/testkit/engine/EngineTestKit.html)` which provides static factory methods named `engine()` and `execute()`.
It is recommended that you select one of the `engine()` variants to benefit from the
fluent API for building a `LauncherDiscoveryRequest`.

|   |If you prefer to use the `LauncherDiscoveryRequestBuilder` from the `Launcher` API<br/>to build your `LauncherDiscoveryRequest`, you must use one of the `execute()` variants in`EngineTestKit`.|
|---|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

The following test class written using JUnit Jupiter will be used in subsequent examples.

```
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

import example.util.Calculator;

import org.junit.jupiter.api.Disabled;
import org.junit.jupiter.api.MethodOrderer.OrderAnnotation;
import org.junit.jupiter.api.Order;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestMethodOrder;

@TestMethodOrder(OrderAnnotation.class)
public class ExampleTestCase {

    private final Calculator calculator = new Calculator();

    @Test
    @Disabled("for demonstration purposes")
    @Order(1)
    void skippedTest() {
        // skipped ...
    }

    @Test
    @Order(2)
    void succeedingTest() {
        assertEquals(42, calculator.multiply(6, 7));
    }

    @Test
    @Order(3)
    void abortedTest() {
        assumeTrue("abc".contains("Z"), "abc does not contain Z");
        // aborted ...
    }

    @Test
    @Order(4)
    void failingTest() {
        // The following throws an ArithmeticException: "/ by zero"
        calculator.divide(1, 0);
    }

}
```

For the sake of brevity, the following sections demonstrate how to test JUnit’s own`JupiterTestEngine` whose unique engine ID is `"junit-jupiter"`. If you want to test your
own `TestEngine` implementation, you need to use its unique engine ID. Alternatively, you
may test your own `TestEngine` by supplying an instance of it to the`EngineTestKit.engine(TestEngine)` static factory method.

#### [](#testkit-engine-statistics)6.3.2. Asserting Statistics ####

One of the most common features of the Test Kit is the ability to assert statistics
against events fired during the execution of a `TestPlan`. The following tests demonstrate
how to assert statistics for *containers* and *tests* in the JUnit Jupiter `TestEngine`.
For details on what statistics are available, consult the Javadoc for `[EventStatistics](../api/org.junit.platform.testkit/org/junit/platform/testkit/engine/EventStatistics.html)`.

```
import static org.junit.platform.engine.discovery.DiscoverySelectors.selectClass;

import example.ExampleTestCase;

import org.junit.jupiter.api.Test;
import org.junit.platform.testkit.engine.EngineTestKit;

class EngineTestKitStatisticsDemo {

    @Test
    void verifyJupiterContainerStats() {
        EngineTestKit
            .engine("junit-jupiter") (1)
            .selectors(selectClass(ExampleTestCase.class)) (2)
            .execute() (3)
            .containerEvents() (4)
            .assertStatistics(stats -> stats.started(2).succeeded(2)); (5)
    }

    @Test
    void verifyJupiterTestStats() {
        EngineTestKit
            .engine("junit-jupiter") (1)
            .selectors(selectClass(ExampleTestCase.class)) (2)
            .execute() (3)
            .testEvents() (6)
            .assertStatistics(stats ->
                stats.skipped(1).started(3).succeeded(1).aborted(1).failed(1)); (7)
    }

}
```

|**1**|                  Select the JUnit Jupiter `TestEngine`.                   |
|-----|---------------------------------------------------------------------------|
|**2**|Select the [`ExampleTestCase`](#testkit-engine-ExampleTestCase) test class.|
|**3**|                          Execute the `TestPlan`.                          |
|**4**|                       Filter by *container* events.                       |
|**5**|                 Assert statistics for *container* events.                 |
|**6**|                         Filter by *test* events.                          |
|**7**|                   Assert statistics for *test* events.                    |

|   |In the `verifyJupiterContainerStats()` test method, the counts for the `started` and`succeeded` statistics are `2` since the `JupiterTestEngine` and the[`ExampleTestCase`](#testkit-engine-ExampleTestCase) class are both considered containers.|
|---|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

#### [](#testkit-engine-events)6.3.3. Asserting Events ####

If you find that [asserting statistics](#testkit-engine-statistics) alone is insufficient
for verifying the expected behavior of test execution, you can work directly with the
recorded `[Event](../api/org.junit.platform.testkit/org/junit/platform/testkit/engine/Event.html)` elements and perform assertions against them.

For example, if you want to verify the reason that the `skippedTest()` method in[`ExampleTestCase`](#testkit-engine-ExampleTestCase) was skipped, you can do that as
follows.

|   |The `assertThatEvents()` method in the following example is a shortcut for`org.assertj.core.api.Assertions.assertThat(events.list())` from the [AssertJ](https://assertj.github.io/doc/) assertion<br/>library.<br/><br/>For details on what *conditions* are available for use with AssertJ assertions against<br/>events, consult the Javadoc for `[EventConditions](../api/org.junit.platform.testkit/org/junit/platform/testkit/engine/EventConditions.html)`.|
|---|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

```
import static org.junit.platform.engine.discovery.DiscoverySelectors.selectMethod;
import static org.junit.platform.testkit.engine.EventConditions.event;
import static org.junit.platform.testkit.engine.EventConditions.skippedWithReason;
import static org.junit.platform.testkit.engine.EventConditions.test;

import example.ExampleTestCase;

import org.junit.jupiter.api.Test;
import org.junit.platform.testkit.engine.EngineTestKit;
import org.junit.platform.testkit.engine.Events;

class EngineTestKitSkippedMethodDemo {

    @Test
    void verifyJupiterMethodWasSkipped() {
        String methodName = "skippedTest";

        Events testEvents = EngineTestKit (5)
            .engine("junit-jupiter") (1)
            .selectors(selectMethod(ExampleTestCase.class, methodName)) (2)
            .execute() (3)
            .testEvents(); (4)

        testEvents.assertStatistics(stats -> stats.skipped(1)); (6)

        testEvents.assertThatEvents() (7)
            .haveExactly(1, event(test(methodName),
                skippedWithReason("for demonstration purposes")));
    }

}
```

|**1**|                                                    Select the JUnit Jupiter `TestEngine`.                                                     |
|-----|-----------------------------------------------------------------------------------------------------------------------------------------------|
|**2**|                   Select the `skippedTest()` method in the [`ExampleTestCase`](#testkit-engine-ExampleTestCase) test class.                   |
|**3**|                                                            Execute the `TestPlan`.                                                            |
|**4**|                                                           Filter by *test* events.                                                            |
|**5**|                                                 Save the *test* `Events` to a local variable.                                                 |
|**6**|                                                  Optionally assert the expected statistics.                                                   |
|**7**|Assert that the recorded *test* events contain exactly one skipped test named`skippedTest` with `"for demonstration purposes"` as the *reason*.|

If you want to verify the type of exception thrown from the `failingTest()` method in[`ExampleTestCase`](#testkit-engine-ExampleTestCase), you can do that as follows.

|   |For details on what *conditions* are available for use with AssertJ assertions against<br/>events and execution results, consult the Javadoc for `[EventConditions](../api/org.junit.platform.testkit/org/junit/platform/testkit/engine/EventConditions.html)` and`[TestExecutionResultConditions](../api/org.junit.platform.testkit/org/junit/platform/testkit/engine/TestExecutionResultConditions.html)`, respectively.|
|---|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

```
import static org.junit.platform.engine.discovery.DiscoverySelectors.selectClass;
import static org.junit.platform.testkit.engine.EventConditions.event;
import static org.junit.platform.testkit.engine.EventConditions.finishedWithFailure;
import static org.junit.platform.testkit.engine.EventConditions.test;
import static org.junit.platform.testkit.engine.TestExecutionResultConditions.instanceOf;
import static org.junit.platform.testkit.engine.TestExecutionResultConditions.message;

import example.ExampleTestCase;

import org.junit.jupiter.api.Test;
import org.junit.platform.testkit.engine.EngineTestKit;

class EngineTestKitFailedMethodDemo {

    @Test
    void verifyJupiterMethodFailed() {
        EngineTestKit.engine("junit-jupiter") (1)
            .selectors(selectClass(ExampleTestCase.class)) (2)
            .execute() (3)
            .testEvents() (4)
            .assertThatEvents().haveExactly(1, (5)
                event(test("failingTest"),
                    finishedWithFailure(
                        instanceOf(ArithmeticException.class), message("/ by zero"))));
    }

}
```

|**1**|                                                                          Select the JUnit Jupiter `TestEngine`.                                                                           |
|-----|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|**2**|                                                        Select the [`ExampleTestCase`](#testkit-engine-ExampleTestCase) test class.                                                        |
|**3**|                                                                                  Execute the `TestPlan`.                                                                                  |
|**4**|                                                                                 Filter by *test* events.                                                                                  |
|**5**|Assert that the recorded *test* events contain exactly one failing test named`failingTest` with an exception of type `ArithmeticException` and an error message<br/>equal to `"/ by zero"`.|

Although typically unnecessary, there are times when you need to verify **all** of the
events fired during the execution of a `TestPlan`. The following test demonstrates how to
achieve this via the `assertEventsMatchExactly()` method in the `EngineTestKit` API.

|   |Since `assertEventsMatchExactly()` matches conditions exactly in the order in which the<br/>events were fired, [`ExampleTestCase`](#testkit-engine-ExampleTestCase) has been<br/>annotated with `@TestMethodOrder(OrderAnnotation.class)` and each test method has been<br/>annotated with `@Order(…​)`. This allows us to enforce the order in which the test<br/>methods are executed, which in turn allows our `verifyAllJupiterEvents()` test to be<br/>reliable.|
|---|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

If you want to do a *partial* match *with* or *without* ordering requirements, you can use
the methods `assertEventsMatchLooselyInOrder()` and `assertEventsMatchLoosely()`,
respectively.

```
import static org.junit.platform.engine.discovery.DiscoverySelectors.selectClass;
import static org.junit.platform.testkit.engine.EventConditions.abortedWithReason;
import static org.junit.platform.testkit.engine.EventConditions.container;
import static org.junit.platform.testkit.engine.EventConditions.engine;
import static org.junit.platform.testkit.engine.EventConditions.event;
import static org.junit.platform.testkit.engine.EventConditions.finishedSuccessfully;
import static org.junit.platform.testkit.engine.EventConditions.finishedWithFailure;
import static org.junit.platform.testkit.engine.EventConditions.skippedWithReason;
import static org.junit.platform.testkit.engine.EventConditions.started;
import static org.junit.platform.testkit.engine.EventConditions.test;
import static org.junit.platform.testkit.engine.TestExecutionResultConditions.instanceOf;
import static org.junit.platform.testkit.engine.TestExecutionResultConditions.message;

import java.io.StringWriter;
import java.io.Writer;

import example.ExampleTestCase;

import org.junit.jupiter.api.Test;
import org.junit.platform.testkit.engine.EngineTestKit;
import org.opentest4j.TestAbortedException;

class EngineTestKitAllEventsDemo {

    @Test
    void verifyAllJupiterEvents() {
        Writer writer = // create a java.io.Writer for debug output

        EngineTestKit.engine("junit-jupiter") (1)
            .selectors(selectClass(ExampleTestCase.class)) (2)
            .execute() (3)
            .allEvents() (4)
            .debug(writer) (5)
            .assertEventsMatchExactly( (6)
                event(engine(), started()),
                event(container(ExampleTestCase.class), started()),
                event(test("skippedTest"), skippedWithReason("for demonstration purposes")),
                event(test("succeedingTest"), started()),
                event(test("succeedingTest"), finishedSuccessfully()),
                event(test("abortedTest"), started()),
                event(test("abortedTest"),
                    abortedWithReason(instanceOf(TestAbortedException.class),
                        message(m -> m.contains("abc does not contain Z")))),
                event(test("failingTest"), started()),
                event(test("failingTest"), finishedWithFailure(
                    instanceOf(ArithmeticException.class), message("/ by zero"))),
                event(container(ExampleTestCase.class), finishedSuccessfully()),
                event(engine(), finishedSuccessfully()));
    }

}
```

|**1**|                                                                Select the JUnit Jupiter `TestEngine`.                                                                |
|-----|----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|**2**|                                             Select the [`ExampleTestCase`](#testkit-engine-ExampleTestCase) test class.                                              |
|**3**|                                                                       Execute the `TestPlan`.                                                                        |
|**4**|                                                                       Filter by *all* events.                                                                        |
|**5**|Print all events to the supplied `writer` for debugging purposes. Debug information<br/>can also be written to an `OutputStream` such as `System.out` or `System.err`.|
|**6**|                                        Assert *all* events in exactly the order in which they were fired by the test engine.                                         |

The `debug()` invocation from the preceding example results in output similar to the
following.

```
All Events:
    Event [type = STARTED, testDescriptor = JupiterEngineDescriptor: [engine:junit-jupiter], timestamp = 2018-12-14T12:45:14.082280Z, payload = null]
    Event [type = STARTED, testDescriptor = ClassTestDescriptor: [engine:junit-jupiter]/[class:example.ExampleTestCase], timestamp = 2018-12-14T12:45:14.089339Z, payload = null]
    Event [type = SKIPPED, testDescriptor = TestMethodTestDescriptor: [engine:junit-jupiter]/[class:example.ExampleTestCase]/[method:skippedTest()], timestamp = 2018-12-14T12:45:14.094314Z, payload = 'for demonstration purposes']
    Event [type = STARTED, testDescriptor = TestMethodTestDescriptor: [engine:junit-jupiter]/[class:example.ExampleTestCase]/[method:succeedingTest()], timestamp = 2018-12-14T12:45:14.095182Z, payload = null]
    Event [type = FINISHED, testDescriptor = TestMethodTestDescriptor: [engine:junit-jupiter]/[class:example.ExampleTestCase]/[method:succeedingTest()], timestamp = 2018-12-14T12:45:14.104922Z, payload = TestExecutionResult [status = SUCCESSFUL, throwable = null]]
    Event [type = STARTED, testDescriptor = TestMethodTestDescriptor: [engine:junit-jupiter]/[class:example.ExampleTestCase]/[method:abortedTest()], timestamp = 2018-12-14T12:45:14.106121Z, payload = null]
    Event [type = FINISHED, testDescriptor = TestMethodTestDescriptor: [engine:junit-jupiter]/[class:example.ExampleTestCase]/[method:abortedTest()], timestamp = 2018-12-14T12:45:14.109956Z, payload = TestExecutionResult [status = ABORTED, throwable = org.opentest4j.TestAbortedException: Assumption failed: abc does not contain Z]]
    Event [type = STARTED, testDescriptor = TestMethodTestDescriptor: [engine:junit-jupiter]/[class:example.ExampleTestCase]/[method:failingTest()], timestamp = 2018-12-14T12:45:14.110680Z, payload = null]
    Event [type = FINISHED, testDescriptor = TestMethodTestDescriptor: [engine:junit-jupiter]/[class:example.ExampleTestCase]/[method:failingTest()], timestamp = 2018-12-14T12:45:14.111217Z, payload = TestExecutionResult [status = FAILED, throwable = java.lang.ArithmeticException: / by zero]]
    Event [type = FINISHED, testDescriptor = ClassTestDescriptor: [engine:junit-jupiter]/[class:example.ExampleTestCase], timestamp = 2018-12-14T12:45:14.113731Z, payload = TestExecutionResult [status = SUCCESSFUL, throwable = null]]
    Event [type = FINISHED, testDescriptor = JupiterEngineDescriptor: [engine:junit-jupiter], timestamp = 2018-12-14T12:45:14.113806Z, payload = TestExecutionResult [status = SUCCESSFUL, throwable = null]]
```

### [](#launcher-api)6.4. JUnit Platform Launcher API ###

One of the prominent goals of JUnit 5 is to make the interface between JUnit and its
programmatic clients – build tools and IDEs – more powerful and stable. The purpose is to
decouple the internals of discovering and executing tests from all the filtering and
configuration that’s necessary from the outside.

JUnit 5 introduces the concept of a `Launcher` that can be used to discover, filter, and
execute tests. Moreover, third party test libraries – like Spock, Cucumber, and FitNesse
– can plug into the JUnit Platform’s launching infrastructure by providing a custom[TestEngine](#test-engines).

The launcher API is in the `[junit-platform-launcher](../api/org.junit.platform.launcher/org/junit/platform/launcher/package-summary.html)` module.

An example consumer of the launcher API is the `[ConsoleLauncher](../api/org.junit.platform.console/org/junit/platform/console/ConsoleLauncher.html)` in the`[junit-platform-console](../api/org.junit.platform.console/org/junit/platform/console/package-summary.html)` project.

#### [](#launcher-api-discovery)6.4.1. Discovering Tests ####

Having *test discovery* as a dedicated feature of the platform itself frees IDEs and build
tools from most of the difficulties they had to go through to identify test classes and
test methods in previous versions of JUnit.

Usage Example:

```
import static org.junit.platform.engine.discovery.ClassNameFilter.includeClassNamePatterns;
import static org.junit.platform.engine.discovery.DiscoverySelectors.selectClass;
import static org.junit.platform.engine.discovery.DiscoverySelectors.selectPackage;

import java.io.PrintWriter;
import java.nio.file.Path;
import java.nio.file.Paths;

import org.junit.platform.engine.FilterResult;
import org.junit.platform.engine.TestDescriptor;
import org.junit.platform.launcher.Launcher;
import org.junit.platform.launcher.LauncherDiscoveryListener;
import org.junit.platform.launcher.LauncherDiscoveryRequest;
import org.junit.platform.launcher.LauncherSession;
import org.junit.platform.launcher.LauncherSessionListener;
import org.junit.platform.launcher.PostDiscoveryFilter;
import org.junit.platform.launcher.TestExecutionListener;
import org.junit.platform.launcher.TestPlan;
import org.junit.platform.launcher.core.LauncherConfig;
import org.junit.platform.launcher.core.LauncherDiscoveryRequestBuilder;
import org.junit.platform.launcher.core.LauncherFactory;
import org.junit.platform.launcher.listeners.SummaryGeneratingListener;
import org.junit.platform.launcher.listeners.TestExecutionSummary;
import org.junit.platform.reporting.legacy.xml.LegacyXmlReportGeneratingListener;
```

```
LauncherDiscoveryRequest request = LauncherDiscoveryRequestBuilder.request()
    .selectors(
        selectPackage("com.example.mytests"),
        selectClass(MyTestClass.class)
    )
    .filters(
        includeClassNamePatterns(".*Tests")
    )
    .build();

try (LauncherSession session = LauncherFactory.openSession()) {
    TestPlan testPlan = session.getLauncher().discover(request);

    // ... discover additional test plans or execute tests
}
```

You can select classes, methods, and all classes in a package or even search for all tests
in the class-path or module-path. Discovery takes place across all participating test
engines.

The resulting `TestPlan` is a hierarchical (and read-only) description of all engines,
classes, and test methods that fit the `LauncherDiscoveryRequest`. The client can
traverse the tree, retrieve details about a node, and get a link to the original source
(like class, method, or file position). Every node in the test plan has a *unique ID*that can be used to invoke a particular test or group of tests.

Clients can register one or more `[LauncherDiscoveryListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/LauncherDiscoveryListener.html)` implementations via the`[LauncherDiscoveryRequestBuilder](../api/org.junit.platform.launcher/org/junit/platform/launcher/core/LauncherDiscoveryRequestBuilder.html)` to gain insight into events that occur during test
discovery. By default, the builder registers an "abort on failure" listener that aborts
test discovery after the first discovery failure is encountered. The default`LauncherDiscoveryListener` can be changed via the`junit.platform.discovery.listener.default` [configuration
parameter](#running-tests-config-params).

#### [](#launcher-api-execution)6.4.2. Executing Tests ####

To execute tests, clients can use the same `LauncherDiscoveryRequest` as in the discovery
phase or create a new request. Test progress and reporting can be achieved by registering
one or more `[TestExecutionListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/TestExecutionListener.html)` implementations with the `Launcher` as in the
following example.

```
LauncherDiscoveryRequest request = LauncherDiscoveryRequestBuilder.request()
    .selectors(
        selectPackage("com.example.mytests"),
        selectClass(MyTestClass.class)
    )
    .filters(
        includeClassNamePatterns(".*Tests")
    )
    .build();

SummaryGeneratingListener listener = new SummaryGeneratingListener();

try (LauncherSession session = LauncherFactory.openSession()) {
    Launcher launcher = session.getLauncher();
    // Register a listener of your choice
    launcher.registerTestExecutionListeners(listener);
    // Discover tests and build a test plan
    TestPlan testPlan = launcher.discover(request);
    // Execute test plan
    launcher.execute(testPlan);
    // Alternatively, execute the request directly
    launcher.execute(request);
}

TestExecutionSummary summary = listener.getSummary();
// Do something with the summary...
```

There is no return value for the `execute()` method, but you can use a`TestExecutionListener` to aggregate the results. For examples see the`[SummaryGeneratingListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/listeners/SummaryGeneratingListener.html)`, `[LegacyXmlReportGeneratingListener](../api/org.junit.platform.reporting/org/junit/platform/reporting/legacy/xml/LegacyXmlReportGeneratingListener.html)`, and`[UniqueIdTrackingListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/listeners/UniqueIdTrackingListener.html)`.

|   |All `TestExecutionListener` methods are called sequentially. Methods for start<br/>events are called in registration order while methods for finish events are called in<br/>reverse order.<br/>Test case execution won’t start before all `executionStarted` calls have returned.|
|---|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

#### [](#launcher-api-engines-custom)6.4.3. Registering a TestEngine ####

See the dedicated section on [TestEngine registration](#test-engines-registration) for
details.

#### [](#launcher-api-post-discovery-filters-custom)6.4.4. Registering a PostDiscoveryFilter ####

In addition to specifying post-discovery filters as part of a `[LauncherDiscoveryRequest](../api/org.junit.platform.launcher/org/junit/platform/launcher/LauncherDiscoveryRequest.html)`passed to the `[Launcher](../api/org.junit.platform.launcher/org/junit/platform/launcher/Launcher.html)` API, `[PostDiscoveryFilter](../api/org.junit.platform.launcher/org/junit/platform/launcher/PostDiscoveryFilter.html)` implementations will be discovered
at runtime via Java’s `[ServiceLoader](https://docs.oracle.com/en/java/javase/11/docs/api/java.base/java/util/ServiceLoader.html)` mechanism and automatically applied by the`Launcher` in addition to those that are part of the request.

For example, an `example.CustomTagFilter` class implementing `PostDiscoveryFilter` and
declared within the `/META-INF/services/org.junit.platform.launcher.PostDiscoveryFilter`file is loaded and applied automatically.

#### [](#launcher-api-launcher-session-listeners-custom)6.4.5. Registering a LauncherSessionListener ####

Registered implementations of `[LauncherSessionListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/LauncherSessionListener.html)` are notified when a`[LauncherSession](../api/org.junit.platform.launcher/org/junit/platform/launcher/LauncherSession.html)` is opened (before a `[Launcher](../api/org.junit.platform.launcher/org/junit/platform/launcher/Launcher.html)` first discovers and executes tests)
and closed (when no more tests will be discovered or executed). They can be registered
programmatically via the `[LauncherConfig](../api/org.junit.platform.launcher/org/junit/platform/launcher/core/LauncherConfig.html)` that is passed to the `[LauncherFactory](../api/org.junit.platform.launcher/org/junit/platform/launcher/core/LauncherFactory.html)`, or
they can be discovered at runtime via Java’s `[ServiceLoader](https://docs.oracle.com/en/java/javase/11/docs/api/java.base/java/util/ServiceLoader.html)` mechanism and automatically
registered with `LauncherSession` (unless automatic registration is disabled.)

##### [](#launcher-api-launcher-session-listeners-tool-support)Tool Support #####

The following build tools and IDEs are known to provide full support for `LauncherSession`:

* Gradle 4.6 and later

* Maven Surefire/Failsafe 3.0.0-M6 and later

* IntelliJ IDEA 2017.3 and later

Other tools might also work but have not been tested explicitly.

##### [](#launcher-api-launcher-session-listeners-tool-example-usage)Example Usage #####

A `LauncherSessionListener` is well suited for implementing once-per-JVM setup/teardown
behavior since it’s called before the first and after the last test in a launcher session,
respectively. The scope of a launcher session depends on the used IDE or build tool but
usually corresponds to the lifecycle of the test JVM. A custom listener that starts an
HTTP server before executing the first test and stops it after the last test has been
executed, could look like this:

src/test/java/example/session/GlobalSetupTeardownListener.java

```
package example.session;

import static java.net.InetAddress.getLoopbackAddress;

import java.io.IOException;
import java.io.UncheckedIOException;
import java.net.InetSocketAddress;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

import com.sun.net.httpserver.HttpServer;

import org.junit.platform.launcher.LauncherSession;
import org.junit.platform.launcher.LauncherSessionListener;
import org.junit.platform.launcher.TestExecutionListener;
import org.junit.platform.launcher.TestPlan;

public class GlobalSetupTeardownListener implements LauncherSessionListener {

    private Fixture fixture;

    @Override
    public void launcherSessionOpened(LauncherSession session) {
        // Avoid setup for test discovery by delaying it until tests are about to be executed
        session.getLauncher().registerTestExecutionListeners(new TestExecutionListener() {
            @Override
            public void testPlanExecutionStarted(TestPlan testPlan) {
                if (fixture == null) {
                    fixture = new Fixture();
                    fixture.setUp();
                }
            }
        });
    }

    @Override
    public void launcherSessionClosed(LauncherSession session) {
        if (fixture != null) {
            fixture.tearDown();
            fixture = null;
        }
    }

    static class Fixture {

        private HttpServer server;
        private ExecutorService executorService;

        void setUp() {
            try {
                server = HttpServer.create(new InetSocketAddress(getLoopbackAddress(), 0), 0);
            }
            catch (IOException e) {
                throw new UncheckedIOException("Failed to start HTTP server", e);
            }
            server.createContext("/test", exchange -> {
                exchange.sendResponseHeaders(204, -1);
                exchange.close();
            });
            executorService = Executors.newCachedThreadPool();
            server.setExecutor(executorService);
            server.start(); (1)
            int port = server.getAddress().getPort();
            System.setProperty("http.server.host", getLoopbackAddress().getHostAddress()); (2)
            System.setProperty("http.server.port", String.valueOf(port)); (3)
        }

        void tearDown() {
            server.stop(0); (4)
            executorService.shutdownNow();
        }
    }

}
```

|**1**|                        Start the HTTP server                        |
|-----|---------------------------------------------------------------------|
|**2**|Export its host address as a system property for consumption by tests|
|**3**|    Export its port as a system property for consumption by tests    |
|**4**|                        Stop the HTTP server                         |

This sample uses the HTTP server implementation from the jdk.httpserver module that comes
with the JDK but would work similarly with any other server or resource. In order for the
listener to be picked up by JUnit Platform, you need to register it as a service by adding
a resource file with the following name and contents to your test runtime classpath (e.g.
by adding the file to `src/test/resources`):

src/test/resources/META-INF/services/org.junit.platform.launcher.LauncherSessionListener

```
example.session.GlobalSetupTeardownListener
```

You can now use the resource from your test:

src/test/java/example/session/HttpTests.java

```
package example.session;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.net.HttpURLConnection;
import java.net.URI;
import java.net.URL;

import org.junit.jupiter.api.Test;

class HttpTests {

    @Test
    void respondsWith204() throws Exception {
        String host = System.getProperty("http.server.host"); (1)
        String port = System.getProperty("http.server.port"); (2)
        URL url = URI.create("http://" + host + ":" + port + "/test").toURL();

        HttpURLConnection connection = (HttpURLConnection) url.openConnection();
        connection.setRequestMethod("GET");
        int responseCode = connection.getResponseCode(); (3)

        assertEquals(204, responseCode); (4)
    }
}
```

|**1**|Read the host address of the server from the system property set by the listener|
|-----|--------------------------------------------------------------------------------|
|**2**|    Read the port of the server from the system property set by the listener    |
|**3**|                          Send a request to the server                          |
|**4**|                     Check the status code of the response                      |

#### [](#launcher-api-launcher-interceptors-custom)6.4.6. Registering a LauncherInterceptor ####

In order to intercept the creation of instances of `[Launcher](../api/org.junit.platform.launcher/org/junit/platform/launcher/Launcher.html)` and`[LauncherSessionListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/LauncherSessionListener.html)` and calls to the `discover` and `execute` methods of the
former, clients can register custom implementations of `[LauncherInterceptor](../api/org.junit.platform.launcher/org/junit/platform/launcher/LauncherInterceptor.html)` via Java’s`[ServiceLoader](https://docs.oracle.com/en/java/javase/11/docs/api/java.base/java/util/ServiceLoader.html)` mechanism by additionally setting the`junit.platform.launcher.interceptors.enabled` [configuration parameter](#running-tests-config-params) to `true`.

A typical use case is to create a custom replace the `ClassLoader` used by the JUnit
Platform to load test classes and engine implementations.

```
import java.io.IOException;
import java.io.UncheckedIOException;
import java.net.URI;
import java.net.URL;
import java.net.URLClassLoader;

import org.junit.platform.launcher.LauncherInterceptor;

public class CustomLauncherInterceptor implements LauncherInterceptor {

    private final URLClassLoader customClassLoader;

    public CustomLauncherInterceptor() throws Exception {
        ClassLoader parent = Thread.currentThread().getContextClassLoader();
        customClassLoader = new URLClassLoader(new URL[] { URI.create("some.jar").toURL() }, parent);
    }

    @Override
    public <T> T intercept(Invocation<T> invocation) {
        Thread currentThread = Thread.currentThread();
        ClassLoader originalClassLoader = currentThread.getContextClassLoader();
        currentThread.setContextClassLoader(customClassLoader);
        try {
            return invocation.proceed();
        }
        finally {
            currentThread.setContextClassLoader(originalClassLoader);
        }
    }

    @Override
    public void close() {
        try {
            customClassLoader.close();
        }
        catch (IOException e) {
            throw new UncheckedIOException("Failed to close custom class loader", e);
        }
    }
}
```

#### [](#launcher-api-launcher-discovery-listeners-custom)6.4.7. Registering a LauncherDiscoveryListener ####

In addition to specifying discovery listeners as part of a `[LauncherDiscoveryRequest](../api/org.junit.platform.launcher/org/junit/platform/launcher/LauncherDiscoveryRequest.html)` or
registering them programmatically via the `[Launcher](../api/org.junit.platform.launcher/org/junit/platform/launcher/Launcher.html)` API, custom`LauncherDiscoveryListener` implementations can be discovered at runtime via Java’s`[ServiceLoader](https://docs.oracle.com/en/java/javase/11/docs/api/java.base/java/util/ServiceLoader.html)` mechanism and automatically registered with the `Launcher` created via
the `[LauncherFactory](../api/org.junit.platform.launcher/org/junit/platform/launcher/core/LauncherFactory.html)`.

For example, an `example.CustomLauncherDiscoveryListener` class implementing`LauncherDiscoveryListener` and declared within the`/META-INF/services/org.junit.platform.launcher.LauncherDiscoveryListener` file is loaded
and registered automatically.

#### [](#launcher-api-listeners-custom)6.4.8. Registering a TestExecutionListener ####

In addition to the public `[Launcher](../api/org.junit.platform.launcher/org/junit/platform/launcher/Launcher.html)` API method for registering test execution listeners
programmatically, custom `[TestExecutionListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/TestExecutionListener.html)` implementations will be discovered at
runtime via Java’s `[ServiceLoader](https://docs.oracle.com/en/java/javase/11/docs/api/java.base/java/util/ServiceLoader.html)` mechanism and automatically registered with the`Launcher` created via the `[LauncherFactory](../api/org.junit.platform.launcher/org/junit/platform/launcher/core/LauncherFactory.html)`.

For example, an `example.CustomTestExecutionListener` class implementing`TestExecutionListener` and declared within the`/META-INF/services/org.junit.platform.launcher.TestExecutionListener` file is loaded and
registered automatically.

#### [](#launcher-api-listeners-config)6.4.9. Configuring a TestExecutionListener ####

When a `[TestExecutionListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/TestExecutionListener.html)` is registered programmatically via the `[Launcher](../api/org.junit.platform.launcher/org/junit/platform/launcher/Launcher.html)` API,
the listener may provide programmatic ways for it to be configured — for example, via its
constructor, setter methods, etc. However, when a `TestExecutionListener` is registered
automatically via Java’s `ServiceLoader` mechanism (see[Registering a TestExecutionListener](#launcher-api-listeners-custom)), there is no way for the user to directly configure the
listener. In such cases, the author of a `TestExecutionListener` may choose to make the
listener configurable via [configuration parameters](#running-tests-config-params). The
listener can then access the configuration parameters via the `TestPlan` supplied to the`testPlanExecutionStarted(TestPlan)` and `testPlanExecutionFinished(TestPlan)` callback
methods. See the `[UniqueIdTrackingListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/listeners/UniqueIdTrackingListener.html)` for an example.

#### [](#launcher-api-listeners-custom-deactivation)6.4.10. Deactivating a TestExecutionListener ####

Sometimes it can be useful to run a test suite *without* certain execution listeners being
active. For example, you might have custom a `[TestExecutionListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/TestExecutionListener.html)` that sends the test
results to an external system for reporting purposes, and while debugging you might not
want these *debug* results to be reported. To do this, provide a pattern for the`junit.platform.execution.listeners.deactivate` *configuration parameter* to specify which
execution listeners should be deactivated (i.e. not registered) for the current test run.

|   |Only listeners registered via the `[ServiceLoader](https://docs.oracle.com/en/java/javase/11/docs/api/java.base/java/util/ServiceLoader.html)` mechanism within the`/META-INF/services/org.junit.platform.launcher.TestExecutionListener` file can be<br/>deactivated. In other words, any `TestExecutionListener` registered explicitly via the`[LauncherDiscoveryRequest](../api/org.junit.platform.launcher/org/junit/platform/launcher/LauncherDiscoveryRequest.html)` cannot be deactivated via the`junit.platform.execution.listeners.deactivate` *configuration parameter*.<br/><br/>In addition, since execution listeners are registered before the test run starts, the`junit.platform.execution.listeners.deactivate` *configuration parameter* can only be<br/>supplied as a JVM system property or via the JUnit Platform configuration file (see[Configuration Parameters](#running-tests-config-params) for details). This *configuration parameter* cannot be<br/>supplied in the `LauncherDiscoveryRequest` that is passed to the `[Launcher](../api/org.junit.platform.launcher/org/junit/platform/launcher/Launcher.html)`.|
|---|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

##### [](#launcher-api-listeners-custom-deactivation-pattern)Pattern Matching Syntax #####

Refer to [Pattern Matching Syntax](#running-tests-config-params-deactivation-pattern) for details.

#### [](#launcher-api-launcher-config)6.4.11. Configuring the Launcher ####

If you require fine-grained control over automatic detection and registration of test
engines and listeners, you may create an instance of `[LauncherConfig](../api/org.junit.platform.launcher/org/junit/platform/launcher/core/LauncherConfig.html)` and supply that to
the `[LauncherFactory](../api/org.junit.platform.launcher/org/junit/platform/launcher/core/LauncherFactory.html)`. Typically, an instance of `LauncherConfig` is created via the
built-in fluent *builder* API, as demonstrated in the following example.

```
LauncherConfig launcherConfig = LauncherConfig.builder()
    .enableTestEngineAutoRegistration(false)
    .enableLauncherSessionListenerAutoRegistration(false)
    .enableLauncherDiscoveryListenerAutoRegistration(false)
    .enablePostDiscoveryFilterAutoRegistration(false)
    .enableTestExecutionListenerAutoRegistration(false)
    .addTestEngines(new CustomTestEngine())
    .addLauncherSessionListeners(new CustomLauncherSessionListener())
    .addLauncherDiscoveryListeners(new CustomLauncherDiscoveryListener())
    .addPostDiscoveryFilters(new CustomPostDiscoveryFilter())
    .addTestExecutionListeners(new LegacyXmlReportGeneratingListener(reportsDir, out))
    .addTestExecutionListeners(new CustomTestExecutionListener())
    .build();

LauncherDiscoveryRequest request = LauncherDiscoveryRequestBuilder.request()
    .selectors(selectPackage("com.example.mytests"))
    .build();

try (LauncherSession session = LauncherFactory.openSession(launcherConfig)) {
    session.getLauncher().execute(request);
}
```

#### [](#launcher-api-dry-run-mode)6.4.12. Dry-Run Mode ####

When running tests via the `[Launcher](../api/org.junit.platform.launcher/org/junit/platform/launcher/Launcher.html)` API, you can enable *dry-run mode* by setting the`junit.platform.execution.dryRun.enabled` [configuration parameter](#running-tests-config-params) to `true`. In this mode, the `[Launcher](../api/org.junit.platform.launcher/org/junit/platform/launcher/Launcher.html)` will not actually
execute any tests but will notify registered `[TestExecutionListener](../api/org.junit.platform.launcher/org/junit/platform/launcher/TestExecutionListener.html)` instances as if all
tests had been skipped and their containers had been successful. This can be useful to
test changes in the configuration of a build or to verify a listener is called as expected
without having to wait for all tests to be executed.

### [](#test-engines)6.5. Test Engines ###

A `TestEngine` facilitates *discovery* and *execution* of tests for a particular
programming model.

For example, JUnit provides a `TestEngine` that discovers and executes tests written using
the JUnit Jupiter programming model (see [Writing Tests](#writing-tests) and [Extension Model](#extensions)).

#### [](#test-engines-junit)6.5.1. JUnit Test Engines ####

JUnit provides three `TestEngine` implementations.

* `[junit-jupiter-engine](../api/org.junit.jupiter.engine/org/junit/jupiter/engine/package-summary.html)`: The core of JUnit Jupiter.

* `[junit-vintage-engine](../api/org.junit.vintage.engine/org/junit/vintage/engine/package-summary.html)`: A thin layer on top of JUnit 4 to allow running *vintage*tests (based on JUnit 3.8 and JUnit 4) with the JUnit Platform launcher infrastructure.

* `[junit-platform-suite-engine](../api/org.junit.platform.suite.engine/org/junit/platform/suite/engine/package-summary.html)`: Executes declarative suites of tests with the JUnit
  Platform launcher infrastructure.

#### [](#test-engines-custom)6.5.2. Custom Test Engines ####

You can contribute your own custom `[TestEngine](../api/org.junit.platform.engine/org/junit/platform/engine/TestEngine.html)` by implementing the interfaces in the[junit-platform-engine](../api/org.junit.platform.engine/org/junit/platform/engine/package-summary.html) module and *registering* your engine.

Every `TestEngine` must provide its own *unique ID*, *discover* tests from an`EngineDiscoveryRequest`, and *execute* those tests according to an `ExecutionRequest`.

|   |The `junit-` unique ID prefix is reserved for TestEngines from the JUnit Team<br/><br/>The JUnit Platform `Launcher` enforces that only `TestEngine` implementations published<br/>by the JUnit Team may use the `junit-` prefix for their `TestEngine` IDs.<br/><br/>* If any third-party `TestEngine` claims to be `junit-jupiter` or `junit-vintage`, an<br/>  exception will be thrown, immediately halting execution of the JUnit Platform.<br/><br/>* If any third-party `TestEngine` uses the `junit-` prefix for its ID, a warning message<br/>  will be logged. Later releases of the JUnit Platform will throw an exception for such<br/>  violations.|
|---|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

In order to facilitate test discovery within IDEs and tools prior to launching the JUnit
Platform, `TestEngine` implementations are encouraged to make use of the `@Testable`annotation. For example, the `@Test` and `@TestFactory` annotations in JUnit Jupiter are
meta-annotated with `@Testable`. Consult the Javadoc for `[@Testable](../api/org.junit.platform.commons/org/junit/platform/commons/annotation/Testable.html)` for further details.

If your custom `TestEngine` needs to be configured, consider allowing users to supply
configuration via [configuration parameters](#running-tests-config-params). Please note,
however, that you are strongly encouraged to use a unique prefix for all configuration
parameters supported by your test engine. Doing so will ensure that there are no conflicts
between the names of your configuration parameters and those from other test engines. In
addition, since configuration parameters may be supplied as JVM system properties, it is
wise to avoid conflicts with the names of other system properties. For example, JUnit
Jupiter uses `junit.jupiter.` as a prefix of all of its supported configuration
parameters. Furthermore, as with the warning above regarding the `junit-` prefix for`TestEngine` IDs, you should not use `junit.` as a prefix for the names of your own
configuration parameters.

Although there is currently no official guide on how to implement a custom `TestEngine`,
you can consult the implementation of [JUnit Test Engines](#test-engines-junit) or the implementation of
third-party test engines listed in the[JUnit 5 wiki](https://github.com/junit-team/junit5/wiki/Third-party-Extensions#junit-platform-test-engines).
You will also find various tutorials and blogs on the Internet that demonstrate how to
write a custom `TestEngine`.

|   |`[HierarchicalTestEngine](../api/org.junit.platform.engine/org/junit/platform/engine/support/hierarchical/HierarchicalTestEngine.html)` is a convenient abstract base implementation of the`TestEngine` SPI (used by the `[junit-jupiter-engine](../api/org.junit.jupiter.engine/org/junit/jupiter/engine/package-summary.html)`) that only requires implementors to<br/>provide the logic for test discovery. It implements execution of `TestDescriptors` that<br/>implement the `Node` interface, including support for parallel execution.|
|---|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

#### [](#test-engines-registration)6.5.3. Registering a TestEngine ####

`TestEngine` registration is supported via Java’s `[ServiceLoader](https://docs.oracle.com/en/java/javase/11/docs/api/java.base/java/util/ServiceLoader.html)` mechanism.

For example, the `junit-jupiter-engine` module registers its`org.junit.jupiter.engine.JupiterTestEngine` in a file named`org.junit.platform.engine.TestEngine` within the `/META-INF/services` folder in the`junit-jupiter-engine` JAR.

#### [](#test-engines-requirements)6.5.4. Requirements ####

|   |The words "must", "must not", "required", "shall", "shall not", "should", "should<br/>not", "recommended", "may", and "optional" in this section are to be interpreted as<br/>described in [RFC 2119.](https://www.ietf.org/rfc/rfc2119.txt)|
|---|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

##### [](#test-engines-requirements-mandatory)Mandatory requirements #####

For interoperability with build tools and IDEs, `TestEngine` implementations must adhere
to the following requirements:

* The `TestDescriptor` returned from `TestEngine.discover()` *must* be the root of a tree
  of `TestDescriptor` instances. This implies that there *must not* be any cycles between
  a node and its descendants.

* A `TestEngine` *must* be able to discover `UniqueIdSelectors` for any unique ID that it
  previously generated and returned from `TestEngine.discover()`. This enables selecting a
  subset of tests to execute or rerun.

* The `executionSkipped`, `executionStarted`, and `executionFinished` methods of the`EngineExecutionListener` passed to `TestEngine.execute()` *must* be called for every`TestDescriptor` node in the tree returned from `TestEngine.discover()` at most
  once. Parent nodes *must* be reported as started before their children and as finished
  after their children. If a node is reported as skipped, there *must not* be any events
  reported for its descendants.

##### [](#test-engines-requirements-enhanced-compatibility)Enhanced compatibility #####

Adhering to the following requirements is optional but recommended for enhanced
compatibility with build tools and IDEs:

* Unless to indicate an empty discovery result, the `TestDescriptor` returned from`TestEngine.discover()` *should* have children rather than being completely dynamic.
  This allows tools to display the structure of the tests and to select a subset of tests
  to execute.

* When resolving `UniqueIdSelectors`, a `TestEngine` *should* only return `TestDescriptor`instances with matching unique IDs including their ancestors but *may* return additional
  siblings or other nodes that are required for the execution of the selected tests.

* `TestEngines` *should* support [tagging](#running-tests-tags) tests and containers so
  that tag filters can be applied when discovering tests.

[](#api-evolution)7. API Evolution
----------

One of the major goals of JUnit 5 is to improve maintainers' capabilities to evolve JUnit
despite its being used in many projects. With JUnit 4 a lot of stuff that was originally
added as an internal construct only got used by external extension writers and tool
builders. That made changing JUnit 4 especially difficult and sometimes impossible.

That’s why JUnit 5 introduces a defined lifecycle for all publicly available interfaces,
classes, and methods.

### [](#api-evolution-version-and-status)7.1. API Version and Status ###

Every published artifact has a version number `<major>.<minor>.<patch>`, and all publicly
available interfaces, classes, and methods are annotated with [@API](https://apiguardian-team.github.io/apiguardian/docs/current/api/) from the[@API Guardian](https://github.com/apiguardian-team/apiguardian) project. The annotation’s `status` attribute can be assigned one of the
following values.

|    Status    |                                                                                                                   Description                                                                                                                   |
|--------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|  `INTERNAL`  |                                                                          Must not be used by any code other than JUnit itself. Might be removed without prior notice.                                                                           |
| `DEPRECATED` |                                                                                      Should no longer be used; might disappear in the next minor release.                                                                                       |
|`EXPERIMENTAL`|Intended for new, experimental features where we are looking for feedback.  <br/> Use this element with caution; it might be promoted to `MAINTAINED` or `STABLE` in the future, but might also be removed without prior notice, even in a patch.|
| `MAINTAINED` |            Intended for features that will not be changed in a backwards- incompatible way for **at least** the next minor release of the current major version. If scheduled for removal, it will be demoted to `DEPRECATED` first.            |
|   `STABLE`   |                                                              Intended for features that will not be changed in a backwards- incompatible way in the current major version (`5.*`).                                                              |

If the `@API` annotation is present on a type, it is considered to be applicable for all
public members of that type as well. A member is allowed to declare a different `status`value of lower stability.

### [](#api-evolution-experimental-apis)7.2. Experimental APIs ###

The following table lists which APIs are currently designated as *experimental* via`@API(status = EXPERIMENTAL)`. Caution should be taken when relying on such APIs.

|              Package Name               |                   Type Name                    |Since |
|-----------------------------------------|------------------------------------------------|------|
|         `org.junit.jupiter.api`         |         `Timeout.ThreadMode` *(enum)*          |`5.9` |
|    `org.junit.jupiter.api.extension`    |    `AnnotatedElementContext` *(interface)*     |`5.10`|
|    `org.junit.jupiter.api.extension`    |  `DynamicTestInvocationContext` *(interface)*  |`5.8` |
|    `org.junit.jupiter.api.extension`    |       `ExecutableInvoker` *(interface)*        |`5.9` |
|    `org.junit.jupiter.api.extension`    |`TestInstancePreConstructCallback` *(interface)*|`5.9` |
|       `org.junit.jupiter.api.io`        |             `CleanupMode` *(enum)*             |`5.9` |
|       `org.junit.jupiter.api.io`        |         `TempDirFactory` *(interface)*         |`5.10`|
|  `org.junit.jupiter.params.converter`   |  `AnnotationBasedArgumentConverter` *(class)*  |`5.10`|
|   `org.junit.jupiter.params.provider`   |  `AnnotationBasedArgumentsProvider` *(class)*  |`5.10`|
|  `org.junit.platform.engine.discovery`  |         `IterationSelector` *(class)*          |`1.9` |
|`org.junit.platform.engine.support.store`|    `NamespacedHierarchicalStore` *(class)*     |`5.10`|
|`org.junit.platform.engine.support.store`|`NamespacedHierarchicalStoreException` *(class)*|`5.10`|
|        `org.junit.platform.jfr`         |  `FlightRecordingDiscoveryListener` *(class)*  |`1.8` |
|        `org.junit.platform.jfr`         |  `FlightRecordingExecutionListener` *(class)*  |`1.8` |
|      `org.junit.platform.launcher`      |   `LauncherDiscoveryListener` *(interface)*    |`1.6` |
|      `org.junit.platform.launcher`      |      `LauncherInterceptor` *(interface)*       |`1.10`|
|      `org.junit.platform.launcher`      |        `TestPlan.Visitor` *(interface)*        |`1.10`|
| `org.junit.platform.launcher.listeners` |      `UniqueIdTrackingListener` *(class)*      |`1.8` |
| `org.junit.platform.reporting.open.xml` |  `OpenTestReportGeneratingListener` *(class)*  |`1.9` |
|     `org.junit.platform.suite.api`      |         `SelectMethod` *(annotation)*          |`1.10`|
|     `org.junit.platform.suite.api`      |         `SelectMethods` *(annotation)*         |`1.10`|

### [](#api-evolution-deprecated-apis)7.3. Deprecated APIs ###

The following table lists which APIs are currently designated as *deprecated* via`@API(status = DEPRECATED)`. You should avoid using deprecated APIs whenever possible,
since such APIs will likely be removed in an upcoming release.

|                  Package Name                  |                Type Name                 |Since|
|------------------------------------------------|------------------------------------------|-----|
|            `org.junit.jupiter.api`             |  `MethodOrderer.Alphanumeric` *(class)*  |`5.7`|
|       `org.junit.platform.commons.util`        |    `BlacklistedExceptions` *(class)*     |`1.7`|
|       `org.junit.platform.commons.util`        |`PreconditionViolationException` *(class)*|`1.5`|
|   `org.junit.platform.engine.support.filter`   |   `ClasspathScanningSupport` *(class)*   |`1.5`|
|`org.junit.platform.engine.support.hierarchical`|      `SingleTestExecutor` *(class)*      |`1.2`|
|    `org.junit.platform.launcher.listeners`     |     `LegacyReportingUtils` *(class)*     |`1.6`|
|          `org.junit.platform.runner`           |        `JUnitPlatform` *(class)*         |`1.8`|
|         `org.junit.platform.suite.api`         |    `UseTechnicalNames` *(annotation)*    |`1.8`|

### [](#api-evolution-tooling)7.4. @API Tooling Support ###

The [@API Guardian](https://github.com/apiguardian-team/apiguardian) project plans to provide tooling support for publishers and consumers
of APIs annotated with [@API](https://apiguardian-team.github.io/apiguardian/docs/current/api/). For example, the tooling support will likely provide a
means to check if JUnit APIs are being used in accordance with `@API` annotation
declarations.

[](#contributors)8. Contributors
----------

Browse the [current list of contributors](https://github.com/junit-team/junit5/graphs/contributors) directly on GitHub.

[](#release-notes)9. Release Notes
----------

The release notes are available [here](../release-notes/index.html#release-notes).

[](#appendix)10. Appendix
----------

### [](#reproducible-builds)10.1. Reproducible Builds ###

Starting with version 5.7, JUnit 5 aims for its non-javadoc JARs to be[reproducible](https://reproducible-builds.org/).

Under identical build conditions, such as Java version, repeated builds should provide the
same output byte-for-byte.

This means that anyone can reproduce the build conditions of the artifacts on Maven
Central/Sonatype and produce the same output artifact locally, confirming that the
artifacts in the repositories were actually generated from this source code.

### [](#dependency-metadata)10.2. Dependency Metadata ###

Artifacts for final releases and milestones are deployed to [Maven Central](https://search.maven.org/), and snapshot
artifacts are deployed to Sonatype’s [snapshots repository](https://oss.sonatype.org/content/repositories/snapshots) under[/org/junit](https://oss.sonatype.org/content/repositories/snapshots/org/junit/).

#### [](#dependency-metadata-junit-platform)10.2.1. JUnit Platform ####

* **Group ID**: `org.junit.platform`

* **Version**: `1.10.2`

* **Artifact IDs**:

  `junit-platform-commons`

  Common APIs and support utilities for the JUnit Platform. Any API annotated with`@API(status = INTERNAL)` is intended solely for usage within the JUnit framework
  itself. *Any usage of internal APIs by external parties is not supported!*

  `junit-platform-console`

  Support for discovering and executing tests on the JUnit Platform from the console.
  See [Console Launcher](#running-tests-console-launcher) for details.

  `junit-platform-console-standalone`

  An executable JAR with all dependencies included is provided in Maven Central under the[junit-platform-console-standalone](https://repo1.maven.org/maven2/org/junit/platform/junit-platform-console-standalone)directory. See [Console Launcher](#running-tests-console-launcher) for details.

  `junit-platform-engine`

  Public API for test engines. See [Registering a TestEngine](#launcher-api-engines-custom) for details.

  `junit-platform-jfr`

   Provides a `LauncherDiscoveryListener` and `TestExecutionListener` for Java Flight
  Recorder events on the JUnit Platform. See [Flight Recorder Support](#running-tests-listeners-flight-recorder)for details.

  `junit-platform-launcher`

  Public API for configuring and launching test plans — typically used by IDEs and
  build tools. See [JUnit Platform Launcher API](#launcher-api) for details.

  `junit-platform-reporting`

  `TestExecutionListener` implementations that generate test reports — typically used
  by IDEs and build tools. See [JUnit Platform Reporting](#junit-platform-reporting) for details.

  `junit-platform-runner`

  Runner for executing tests and test suites on the JUnit Platform in a JUnit 4
  environment. See [Using JUnit 4 to run the JUnit Platform](#running-tests-junit-platform-runner) for details.

  `junit-platform-suite`

   JUnit Platform Suite artifact that transitively pulls in dependencies on `junit-platform-suite-api` and `junit-platform-suite-engine` for simplified dependency
  management in build tools such as Gradle and Maven.

  `junit-platform-suite-api`

  Annotations for configuring test suites on the JUnit Platform. Supported by the[JUnit Platform Suite Engine](#junit-platform-suite-engine) and the[JUnitPlatform runner](#running-tests-junit-platform-runner).

  `junit-platform-suite-commons`

  Common support utilities for executing test suites on the JUnit Platform.

  `junit-platform-suite-engine`

  Engine that executes test suites on the JUnit Platform; only required at runtime. See[JUnit Platform Suite Engine](#junit-platform-suite-engine) for details.

  `junit-platform-testkit`

  Provides support for executing a test plan for a given `TestEngine` and then
  accessing the results via a fluent API to verify the expected results.

#### [](#dependency-metadata-junit-jupiter)10.2.2. JUnit Jupiter ####

* **Group ID**: `org.junit.jupiter`

* **Version**: `5.10.2`

* **Artifact IDs**:

  `junit-jupiter`

  JUnit Jupiter aggregator artifact that transitively pulls in dependencies on`junit-jupiter-api`, `junit-jupiter-params`, and `junit-jupiter-engine` for
  simplified dependency management in build tools such as Gradle and Maven.

  `junit-jupiter-api`

  JUnit Jupiter API for [writing tests](#writing-tests) and [extensions](#extensions).

  `junit-jupiter-engine`

  JUnit Jupiter test engine implementation; only required at runtime.

  `junit-jupiter-params`

  Support for [parameterized tests](#writing-tests-parameterized-tests) in JUnit Jupiter.

  `junit-jupiter-migrationsupport`

  Support for migrating from JUnit 4 to JUnit Jupiter; only required for support for
  JUnit 4’s `@Ignore` annotation and for running selected JUnit 4 rules.

#### [](#dependency-metadata-junit-vintage)10.2.3. JUnit Vintage ####

* **Group ID**: `org.junit.vintage`

* **Version**: `5.10.2`

* **Artifact ID**:

  `junit-vintage-engine`

  JUnit Vintage test engine implementation that allows one to run *vintage* JUnit tests
  on the JUnit Platform. *Vintage* tests include those written using JUnit 3 or JUnit 4
  APIs or tests written using testing frameworks built on those APIs.

#### [](#dependency-metadata-junit-bom)10.2.4. Bill of Materials (BOM) ####

The *Bill of Materials* POM provided under the following Maven coordinates can be used to
ease dependency management when referencing multiple of the above artifacts using[Maven](https://maven.apache.org/guides/introduction/introduction-to-dependency-mechanism.html#Importing_Dependencies)or [Gradle](https://docs.gradle.org/current/userguide/platforms.html#sub:bom_import).

* **Group ID**: `org.junit`

* **Artifact ID**: `junit-bom`

* **Version**: `5.10.2`

#### [](#dependency-metadata-dependencies)10.2.5. Dependencies ####

Most of the above artifacts have a dependency in their published Maven POMs on the
following *@API Guardian* JAR.

* **Group ID**: `org.apiguardian`

* **Artifact ID**: `apiguardian-api`

* **Version**: `1.1.2`

In addition, most of the above artifacts have a direct or transitive dependency on the
following *OpenTest4J* JAR.

* **Group ID**: `org.opentest4j`

* **Artifact ID**: `opentest4j`

* **Version**: `1.3.0`

### [](#dependency-diagram)10.3. Dependency Diagram ###