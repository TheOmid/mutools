import React, { FC } from 'react'

export const NavbarLayout: FC = (props: any) => {
    const { C1, C2, C3, C4, C5 } = props;

    return ( 
        <Box sx={{ flexGrow: 1 }}>
          <Grid container>

            <Grid item xs={1}>
              { C1 }
            </Grid>

            <Grid item xs={1}>
            </Grid>
            
            <Grid item xs={1}>
              { C2 }
            </Grid>

            <Grid item xs={1}>
              { C3 }
            </Grid>

            <Grid item xs={1}>
              { C4 }
            </Grid>

            <Grid item xs={4}>
            </Grid>
            
            <Grid item xs={3}>
              { C5 }
            </Grid>            
            
          </Grid>
        </Box>
    );
}
