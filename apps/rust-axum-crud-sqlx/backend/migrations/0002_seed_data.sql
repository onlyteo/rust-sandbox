INSERT INTO products (
    id,
    name,
    description,
    packing,
    price,
    stock
) VALUES (
    '6821a470-071b-4cf4-b993-43cc8b2a8827',
    'Coffee Beans',
    'Medium roasted coffee beans',
    '1kg bag',
    '€10',
    110
), (
    'e8a7bc56-4f5e-4037-92ed-20761f8333c0',
    'Ground Coffee',
    'Finely ground coffee',
    '500g bag',
    '€6',
    150
), (
    '33433d98-db71-45d4-953d-82fc29f98188',
    'Espresso Capsules',
    'Compatible espresso capsules',
    'Box of 20',
    '€8',
    100
) ON CONFLICT (id) DO NOTHING;
