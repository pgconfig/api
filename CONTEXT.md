# PostgreSQL Configuration Tuning

This context describes how pgconfig turns facts about a PostgreSQL environment
and workload into configuration guidance.

## Language

**Tuning Request**:
The facts about a target PostgreSQL environment and workload from which tuning
recommendations are produced.
_Avoid_: Input, tuning parameters

**Tuning Recommendation**:
A proposed value for one PostgreSQL parameter together with the deterministic
reason that led to that value.
_Avoid_: Configuration value, rule result

**Tuning Assumption**:
An explicit fallback used when a tuning request omits a fact that is not
required to produce recommendations.
_Avoid_: Silent default

**PostgreSQL Version**:
The complete PostgreSQL release identifier for the target environment, such as
`9.6.24` or `18.4`.
_Avoid_: Numeric version, tuning line

**PostgreSQL Major Version**:
The PostgreSQL compatibility series that determines available features and
parameters: two number groups before PostgreSQL 10 and one group from 10 onward.
_Avoid_: Tuning version
