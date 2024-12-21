export interface Feature {
  environments: string[];
}

export interface Config {
  features: Record<string, Feature>;
}
