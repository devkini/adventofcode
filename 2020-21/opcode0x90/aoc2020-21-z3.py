#!/usr/bin/env python3
#
# pip install z3-solver
# https://ericpony.github.io/z3py-tutorial/guide-examples.htm
#
import itertools

import z3


def solve(foods):
    s = z3.Solver()

    # encode all ingredients and allergens
    ingredients_ = list(set(item for pair in foods for item in pair[0]))
    allergens_ = list(set(item for pair in foods for item in pair[1]))

    ingredients = dict((v, i) for i, v in enumerate(ingredients_))
    allergens = dict((v, i) for i, v in enumerate(allergens_))

    # we want to find out which ingredient is an allergen
    # we treat this as an MxN assignment problem
    # ie. solve for which ingredient can be possibly assigned as an allergen, while satisfying all input constrains
    assignments = z3.IntVector('allergen', len(allergens))

    # program the valid range of our ingredient encodings
    for a in assignments:
        s.add(z3.And(a >= 0, a < len(ingredients)))

    # there can only be one possible ingredient assigned to an allergen
    s.add(z3.Distinct(assignments))

    for i, a in foods:

        # program the all possible pairs of assignment for this food
        for a_ in a:
            food = []
            for i_ in i:
                # encode the input constrain
                I = ingredients[i_]
                A = assignments[allergens[a_]]

                # one of these (allergen == ingredient) pair could be valid
                food.append(A == I)
            s.add(z3.Or(food))

        # ensure that this food doesn't contain any other types of allergen
        food = []

        not_a = set(allergens.keys()) - set(a)
        for i_, a_ in itertools.product(i, not_a):
            # encode the input constrain
            I = ingredients[i_]
            A = assignments[allergens[a_]]

            # not any other types of allergen
            # print(f"{(i_, a_)=}")
            food.append(I != A)

        s.add(z3.Or(food))

    # are we asking the impossible?
    print(f"{s=}")
    print()

    r = s.check()
    print(f"{r=}")

    if r == z3.sat:
        # constrain satisfied, now we extract the solution from the model
        m = s.model()
        print(f"{m=}")

        # reverse the our integer encoding into string
        allergen = dict(enumerate(allergens_))
        ingredient = dict(enumerate(ingredients_))

        solution = [{'ingredient': ingredient[m.eval(a).as_long()], 'allergen': allergen[i]}
                    for i, a in enumerate(assignments)]
        return solution


def main(lines):
    deps = []
    for line in lines:
        ingredients, _, allergens = line.partition('(')
        ingredients = ingredients.split()
        allergens = [a.strip() for a in allergens.rstrip(')').partition(' ')[2].split(',')]
        deps.append((ingredients, allergens))

    print(f"{deps=}")
    print()

    s = solve(deps)
    print()

    print(f"{s=}")
    print()

    allergens = sorted(x['ingredient'] for x in s)
    print(f"{allergens=}")
    print()

    part1 = len([x for i, a in deps for x in i if x not in allergens])
    print(f"{part1=}")

    part2 = ','.join(a['ingredient'] for a in sorted(s, key=lambda x: x['allergen']))
    print(f"{part2=}")

    print()


if __name__ == '__main__':
    data = """
        mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
        trh fvjkl sbzzf mxmxvkd (contains dairy)
        sqjhc fvjkl (contains soy)
        sqjhc mxmxvkd sbzzf (contains fish)
    """
    if input("Read from input.txt? (Y/n) ").lower() != "n":
        with open('input.txt', 'r') as f:
            data = f.read()

    main([line.strip() for line in data.strip().splitlines()])
