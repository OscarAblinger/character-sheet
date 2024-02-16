import type { CharacterSheetServices } from './character-sheet-module.js';

/**
 * Register custom validation checks.
 */
export function registerValidationChecks(services: CharacterSheetServices) {
/*
    const registry = services.validation.ValidationRegistry;
    const validator = services.validation.CharacterSheetValidator;
    const checks: ValidationChecks<CharacterSheetAstType> = {
        Person: validator.checkPersonStartsWithCapital
    };
    registry.register(checks, validator);
*/
}

/**
 * Implementation of custom validations.
 */
export class CharacterSheetValidator {

    /*
    checkPersonStartsWithCapital(person: Person, accept: ValidationAcceptor): void {
        if (person.name) {
            const firstChar = person.name.substring(0, 1);
            if (firstChar.toUpperCase() !== firstChar) {
                accept('warning', 'Person name should start with a capital.', { node: person, property: 'name' });
            }
        }
    }
    */
}