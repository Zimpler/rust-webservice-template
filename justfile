# Local Variables:
# mode: makefile
# indent-tabs-mode: nil
# End:
# vim: set ft=make :

# Connect to the database. Note that this requires $DATABASE_URL to be set.
psql:
    psql -d $DATABASE_URL
