CREATE TABLE countries (
    id SERIAL PRIMARY KEY,
    name TEXT,
    alpha2 TEXT,
    alpha3 TEXT,
    region TEXT
);

INSERT INTO
    countries (name, alpha2, alpha3, region)
VALUES
    ('Afghanistan', 'AF', 'AFG', 'Asia'),
    ('Albania', 'AL', 'ALB', 'Europe'),
    ('Algeria', 'DZ', 'DZA', 'Africa'),
    ('Andorra', 'AD', 'AND', 'Europe'),
    ('Angola', 'AO', 'AGO', 'Africa'),
    ('Antigua and Barbuda', 'AG', 'ATG', 'Americas'),
    ('Argentina', 'AR', 'ARG', 'Americas'),
    ('Armenia', 'AM', 'ARM', 'Asia'),
    ('Australia', 'AU', 'AUS', 'Oceania'),
    ('Austria', 'AT', 'AUT', 'Europe'),
    ('Azerbaijan', 'AZ', 'AZE', 'Asia'),
    ('Bahamas', 'BS', 'BHS', 'Americas'),
    ('Bahrain', 'BH', 'BHR', 'Asia'),
    ('Bangladesh', 'BD', 'BGD', 'Asia'),
    ('Barbados', 'BB', 'BRB', 'Americas'),
    ('Belarus', 'BY', 'BLR', 'Europe'),
    ('Belgium', 'BE', 'BEL', 'Europe'),
    ('Belize', 'BZ', 'BLZ', 'Americas'),
    ('Benin', 'BJ', 'BEN', 'Africa'),
    ('Bhutan', 'BT', 'BTN', 'Asia'),
    ('Bolivia', 'BO', 'BOL', 'Americas'),
    ('Bosnia and Herzegovina', 'BA', 'BIH', 'Europe'),
    ('Botswana', 'BW', 'BWA', 'Africa'),
    ('Brazil', 'BR', 'BRA', 'Americas'),
    ('Brunei', 'BN', 'BRN', 'Asia'),
    ('Bulgaria', 'BG', 'BGR', 'Europe'),
    ('Burkina Faso', 'BF', 'BFA', 'Africa'),
    ('Burundi', 'BI', 'BDI', 'Africa'),
    ('Cabo Verde', 'CV', 'CPV', 'Africa'),
    ('Cambodia', 'KH', 'KHM', 'Asia'),
    ('Cameroon', 'CM', 'CMR', 'Africa'),
    ('Canada', 'CA', 'CAN', 'Americas'),
    ('Central African Republic', 'CF', 'CAF', 'Africa'),
    ('Chad', 'TD', 'TCD', 'Africa'),
    ('Chile', 'CL', 'CHL', 'Americas'),
    ('China', 'CN', 'CHN', 'Asia'),
    ('Colombia', 'CO', 'COL', 'Americas'),
    ('Comoros', 'KM', 'COM', 'Africa'),
    (
        'Congo (Congo-Brazzaville)',
        'CG',
        'COG',
        'Africa'
    ),
    ('Costa Rica', 'CR', 'CRI', 'Americas'),
    ('Croatia', 'HR', 'HRV', 'Europe'),
    ('Cuba', 'CU', 'CUB', 'Americas'),
    ('Cyprus', 'CY', 'CYP', 'Europe'),
    ('Czechia (Czech Republic)', 'CZ', 'CZE', 'Europe'),
    (
        'Democratic Republic of the Congo',
        'CD',
        'COD',
        'Africa'
    ),
    ('Denmark', 'DK', 'DNK', 'Europe'),
    ('Djibouti', 'DJ', 'DJI', 'Africa'),
    ('Dominica', 'DM', 'DMA', 'Americas'),
    ('Dominican Republic', 'DO', 'DOM', 'Americas'),
    ('Ecuador', 'EC', 'ECU', 'Americas'),
    ('Egypt', 'EG', 'EGY', 'Africa'),
    ('El Salvador', 'SV', 'SLV', 'Americas'),
    ('Equatorial Guinea', 'GQ', 'GNQ', 'Africa'),
    ('Eritrea', 'ER', 'ERI', 'Africa'),
    ('Estonia', 'EE', 'EST', 'Europe'),
    (
        'Eswatini (fmr. "Swaziland")',
        'SZ',
        'SWZ',
        'Africa'
    ),
    ('Ethiopia', 'ET', 'ETH', 'Africa'),
    ('Fiji', 'FJ', 'FJI', 'Oceania'),
    ('Finland', 'FI', 'FIN', 'Europe'),
    ('France', 'FR', 'FRA', 'Europe'),
    ('Gabon', 'GA', 'GAB', 'Africa'),
    ('Gambia', 'GM', 'GMB', 'Africa'),
    ('Georgia', 'GE', 'GEO', 'Asia'),
    ('Germany', 'DE', 'DEU', 'Europe'),
    ('Ghana', 'GH', 'GHA', 'Africa'),
    ('Greece', 'GR', 'GRC', 'Europe'),
    ('Grenada', 'GD', 'GRD', 'Americas'),
    ('Guatemala', 'GT', 'GTM', 'Americas'),
    ('Guinea', 'GN', 'GIN', 'Africa'),
    ('Guinea-Bissau', 'GW', 'GNB', 'Africa'),
    ('Guyana', 'GY', 'GUY', 'Americas'),
    ('Haiti', 'HT', 'HTI', 'Americas'),
    ('Holy See', 'VA', 'VAT', 'Europe'),
    ('Honduras', 'HN', 'HND', 'Americas'),
    ('Hungary', 'HU', 'HUN', 'Europe'),
    ('Iceland', 'IS', 'ISL', 'Europe'),
    ('India', 'IN', 'IND', 'Asia'),
    ('Indonesia', 'ID', 'IDN', 'Asia'),
    ('Iran', 'IR', 'IRN', 'Asia'),
    ('Iraq', 'IQ', 'IRQ', 'Asia'),
    ('Ireland', 'IE', 'IRL', 'Europe'),
    ('Israel', 'IL', 'ISR', 'Asia'),
    ('Italy', 'IT', 'ITA', 'Europe'),
    ('Jamaica', 'JM', 'JAM', 'Americas'),
    ('Japan', 'JP', 'JPN', 'Asia'),
    ('Jordan', 'JO', 'JOR', 'Asia'),
    ('Kazakhstan', 'KZ', 'KAZ', 'Asia'),
    ('Kenya', 'KE', 'KEN', 'Africa'),
    ('Kiribati', 'KI', 'KIR', 'Oceania'),
    ('Kuwait', 'KW', 'KWT', 'Asia'),
    ('Kyrgyzstan', 'KG', 'KGZ', 'Asia'),
    ('Laos', 'LA', 'LAO', 'Asia'),
    ('Latvia', 'LV', 'LVA', 'Europe'),
    ('Lebanon', 'LB', 'LBN', 'Asia'),
    ('Lesotho', 'LS', 'LSO', 'Africa'),
    ('Liberia', 'LR', 'LBR', 'Africa'),
    ('Libya', 'LY', 'LBY', 'Africa'),
    ('Liechtenstein', 'LI', 'LIE', 'Europe'),
    ('Lithuania', 'LT', 'LTU', 'Europe'),
    ('Luxembourg', 'LU', 'LUX', 'Europe'),
    ('Madagascar', 'MG', 'MDG', 'Africa'),
    ('Malawi', 'MW', 'MWI', 'Africa'),
    ('Malaysia', 'MY', 'MYS', 'Asia'),
    ('Maldives', 'MV', 'MDV', 'Asia'),
    ('Mali', 'ML', 'MLI', 'Africa'),
    ('Malta', 'MT', 'MLT', 'Europe'),
    ('Marshall Islands', 'MH', 'MHL', 'Oceania'),
    ('Mauritania', 'MR', 'MRT', 'Africa'),
    ('Mauritius', 'MU', 'MUS', 'Africa'),
    ('Mexico', 'MX', 'MEX', 'Americas'),
    ('Micronesia', 'FM', 'FSM', 'Oceania'),
    ('Moldova', 'MD', 'MDA', 'Europe'),
    ('Monaco', 'MC', 'MCO', 'Europe'),
    ('Mongolia', 'MN', 'MNG', 'Asia'),
    ('Montenegro', 'ME', 'MNE', 'Europe'),
    ('Morocco', 'MA', 'MAR', 'Africa'),
    ('Mozambique', 'MZ', 'MOZ', 'Africa'),
    ('Myanmar (formerly Burma)', 'MM', 'MMR', 'Asia'),
    ('Namibia', 'NA', 'NAM', 'Africa'),
    ('Nauru', 'NR', 'NRU', 'Oceania'),
    ('Nepal', 'NP', 'NPL', 'Asia'),
    ('Netherlands', 'NL', 'NLD', 'Europe'),
    ('New Zealand', 'NZ', 'NZL', 'Oceania'),
    ('Nicaragua', 'NI', 'NIC', 'Americas'),
    ('Niger', 'NE', 'NER', 'Africa'),
    ('Nigeria', 'NG', 'NGA', 'Africa'),
    ('North Korea', 'KP', 'PRK', 'Asia'),
    (
        'North Macedonia (formerly Macedonia)',
        'MK',
        'MKD',
        'Europe'
    ),
    ('Norway', 'NO', 'NOR', 'Europe'),
    ('Oman', 'OM', 'OMN', 'Asia'),
    ('Pakistan', 'PK', 'PAK', 'Asia'),
    ('Palau', 'PW', 'PLW', 'Oceania'),
    ('Palestine State', 'PS', 'PSE', 'Asia'),
    ('Panama', 'PA', 'PAN', 'Americas'),
    ('Papua New Guinea', 'PG', 'PNG', 'Oceania'),
    ('Paraguay', 'PY', 'PRY', 'Americas'),
    ('Peru', 'PE', 'PER', 'Americas'),
    ('Philippines', 'PH', 'PHL', 'Asia'),
    ('Poland', 'PL', 'POL', 'Europe'),
    ('Portugal', 'PT', 'PRT', 'Europe'),
    ('Qatar', 'QA', 'QAT', 'Asia'),
    ('Romania', 'RO', 'ROU', 'Europe'),
    ('Russia', 'RU', 'RUS', 'Europe'),
    ('Rwanda', 'RW', 'RWA', 'Africa'),
    ('Saint Kitts and Nevis', 'KN', 'KNA', 'Americas'),
    ('Saint Lucia', 'LC', 'LCA', 'Americas'),
    (
        'Saint Vincent and the Grenadines',
        'VC',
        'VCT',
        'Americas'
    ),
    ('Samoa', 'WS', 'WSM', 'Oceania'),
    ('San Marino', 'SM', 'SMR', 'Europe'),
    ('Sao Tome and Principe', 'ST', 'STP', 'Africa'),
    ('Saudi Arabia', 'SA', 'SAU', 'Asia'),
    ('Senegal', 'SN', 'SEN', 'Africa'),
    ('Serbia', 'RS', 'SRB', 'Europe'),
    ('Seychelles', 'SC', 'SYC', 'Africa'),
    ('Sierra Leone', 'SL', 'SLE', 'Africa'),
    ('Singapore', 'SG', 'SGP', 'Asia'),
    ('Slovakia', 'SK', 'SVK', 'Europe'),
    ('Slovenia', 'SI', 'SVN', 'Europe'),
    ('Solomon Islands', 'SB', 'SLB', 'Oceania'),
    ('Somalia', 'SO', 'SOM', 'Africa'),
    ('South Africa', 'ZA', 'ZAF', 'Africa'),
    ('South Korea', 'KR', 'KOR', 'Asia'),
    ('South Sudan', 'SS', 'SSD', 'Africa'),
    ('Spain', 'ES', 'ESP', 'Europe'),
    ('Sri Lanka', 'LK', 'LKA', 'Asia'),
    ('Sudan', 'SD', 'SDN', 'Africa'),
    ('Suriname', 'SR', 'SUR', 'Americas'),
    ('Sweden', 'SE', 'SWE', 'Europe'),
    ('Switzerland', 'CH', 'CHE', 'Europe'),
    ('Syria', 'SY', 'SYR', 'Asia'),
    ('Tajikistan', 'TJ', 'TJK', 'Asia'),
    ('Tanzania', 'TZ', 'TZA', 'Africa'),
    ('Thailand', 'TH', 'THA', 'Asia'),
    ('Timor-Leste', 'TL', 'TLS', 'Asia'),
    ('Togo', 'TG', 'TGO', 'Africa'),
    ('Tonga', 'TO', 'TON', 'Oceania'),
    ('Trinidad and Tobago', 'TT', 'TTO', 'Americas'),
    ('Tunisia', 'TN', 'TUN', 'Africa'),
    ('Turkey', 'TR', 'TUR', 'Asia'),
    ('Turkmenistan', 'TM', 'TKM', 'Asia'),
    ('Tuvalu', 'TV', 'TUV', 'Oceania'),
    ('Uganda', 'UG', 'UGA', 'Africa'),
    ('Ukraine', 'UA', 'UKR', 'Europe'),
    ('United Arab Emirates', 'AE', 'ARE', 'Asia'),
    ('United Kingdom', 'GB', 'GBR', 'Europe'),
    (
        'United States of America',
        'US',
        'USA',
        'Americas'
    ),
    ('Uruguay', 'UY', 'URY', 'Americas'),
    ('Uzbekistan', 'UZ', 'UZB', 'Asia'),
    ('Vanuatu', 'VU', 'VUT', 'Oceania'),
    ('Venezuela', 'VE', 'VEN', 'Americas'),
    ('Vietnam', 'VN', 'VNM', 'Asia'),
    ('Yemen', 'YE', 'YEM', 'Asia'),
    ('Zambia', 'ZM', 'ZMB', 'Africa'),
    ('Zimbabwe', 'ZW', 'ZWE', 'Africa');
