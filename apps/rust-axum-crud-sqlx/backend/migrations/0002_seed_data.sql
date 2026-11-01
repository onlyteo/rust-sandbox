INSERT INTO products (
    id,
    name,
    description,
    packing,
    price,
    stock
) VALUES (
    'd2ac5e0c-83b0-4934-a975-a2134fc1e767',
    'Coffee Beans',
    'Lightly roasted coffee beans',
    '500g bag',
    '€10',
    87
), (
    '138c8f8a-1bfd-411a-8b7a-3a67870e6ee8',
    'Coffee Beans',
    'Medium roasted coffee beans',
    '500g bag',
    '€10',
    110
), (
    '6821a470-071b-4cf4-b993-43cc8b2a8827',
    'Coffee Beans',
    'Dark roasted coffee beans',
    '500g bag',
    '€10',
    55
), (
    'e8a7bc56-4f5e-4037-92ed-20761f8333c0',
    'Ground Espresso',
    'Finely ground Espresso coffee',
    '250g bag',
    '€8',
    150
), (
    '33433d98-db71-45d4-953d-82fc29f98188',
    'Espresso Capsules',
    'Compatible espresso capsules',
    'Box of 20',
    '€12',
    100
), (
    '6292a7d4-8a60-4bac-a971-c22d1e0e93be',
    'Coffee Filters',
    'Filters for coffee machine',
    'Box of 100',
    '€11',
    90
) ON CONFLICT (id) DO NOTHING;
