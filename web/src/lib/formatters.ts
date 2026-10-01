/** What the API documents about one PostgreSQL parameter. */
export type ParameterDocumentation = {
  abstract?: string;
  recomendations?: Record<string, string>;
  type?: string;
  details?: string[];
  url?: string;
  default_value?: string;
};

export type Parameter = {
  name: string;
  config_value: string | number;
  format?: string;
  documentation?: ParameterDocumentation;
};

export type Category = {
  category: string;
  description: string;
  parameters: Parameter[];
};

/** One entry of `get-config-all-environments`: a profile and its categories. */
export type EnvironmentConfig = {
  environment: string;
  configuration: Category[];
};

/** A parameter with one value per profile, keyed by the lowercase profile. */
export type ComparisonParam = {
  name: string;
  documentation: ParameterDocumentation | undefined;
  [environment: string]: unknown;
};

export type ComparisonCategory = {
  category: string;
  name: string;
  params: ComparisonParam[];
};

export const createComparisonStructure = (
  configs: EnvironmentConfig[],
): ComparisonCategory[] => {
  if (configs.length > 0) {
    return configs[0].configuration.map(({ category, description: name }) => ({
      category,
      name,
      params: [],
    }));
  }

  return [];
};

export const formatConfigs = (
  configsFromBack: EnvironmentConfig[],
): ComparisonCategory[] => {
  if (configsFromBack.length <= 0) {
    return [];
  }

  return configsFromBack.reduce((acc, { configuration: config, environment: env }) => {
    config.forEach(({ category, parameters }) => {
      const categoryFound = acc.find((it) => it.category === category);
      if (!categoryFound) return;

      parameters.forEach(
        ({ name: paramName, config_value: paramValue, documentation }) => {
          const populatedParamIndex = categoryFound.params.findIndex(
            (it) => it.name === paramName,
          );

          if (populatedParamIndex === -1) {
            categoryFound.params.push({
              name: paramName,
              [env.toLowerCase()]: paramValue,
              documentation,
            });
          } else {
            categoryFound.params[populatedParamIndex] = {
              ...categoryFound.params[populatedParamIndex],
              [env.toLowerCase()]: paramValue,
            };
          }
        },
      );
    });
    return acc;
  }, createComparisonStructure(configsFromBack));
};
