#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod reportes_view {

    use ink::prelude::vec::Vec;
    use marketplace::marketplace::MarketplaceRef;
    use marketplace::marketplace::{Publicacion, Producto, Usuario, ErrorSistema, Rol, Categoria};

    #[ink(storage)]
    pub struct ReportesView {
        marketplace: MarketplaceRef,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, Clone, PartialEq)]
    pub struct EstadisticaCategoria {
        categoria: Categoria,
        total_ventas: u32,
        //calificacion_promedio: u32,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, Clone, PartialEq)]
    pub struct EstadisticaUsuario {
        usuario: Usuario,
        cant_ordenes: u32,
    }

    impl EstadisticaUsuario {
        pub fn get_usuario_id(&self) -> AccountId {
            self.usuario.get_id()
        }
    }

    impl ReportesView {
        /// Constructor que inicializa el contrato 
        #[ink(constructor)]
        pub fn new(marketplace_code_hash: Hash) -> Self {
            let marketplace = MarketplaceRef::new()
                .code_hash(marketplace_code_hash)
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();

            Self { marketplace }
        }


        /// Retorna los cinco vendedores con mayor reputación.
        /// 
        /// Filtra los usuarios cuyo rol es `Vendedor`, los ordena en forma
        /// descendente según su reputación como vendedores y devuelve los
        /// primeros cinco (o menos si no hay tantos).
        ///
        /// # Retorna
        /// - Un `Vec<Usuario>` con hasta cinco vendedores mejor posicionados.
        #[ink(message)]
        pub fn get_top_cinco_vendedores(&self) -> Vec<Usuario> {
            let mut vendedores: Vec<Usuario> = self
                .marketplace
                .get_lista_usuarios()
                .into_iter()
                .filter(|u| u.get_rol() == marketplace::marketplace::Rol::Vendedor)
                .collect();

            // Ordenar por reputación descendente
            vendedores.sort_by(|a, b| {
                b.get_reputacion_como_vendedor()
                    .cmp(&a.get_reputacion_como_vendedor())
            });

            // Tomar los primeros 5 (o menos)
            vendedores.into_iter().take(5).collect()
        }

        /// Retorna los cinco compradores con mayor reputación.
        /// 
        /// Filtra los usuarios cuyo rol es `Comprador`, los ordena en forma
        /// descendente según su reputación como compradores y devuelve los
        /// primeros cinco (o menos si no hay tantos).
        ///
        /// # Retorna
        /// - Un `Vec<Usuario>` con hasta cinco compradores mejor posicionados.
        #[ink(message)]
        pub fn get_top_cinco_compradores(&self) -> Vec<Usuario> {
            let mut compradores: Vec<Usuario> = self
                .marketplace
                .get_lista_usuarios()
                .into_iter()
                .filter(|u| u.get_rol() == marketplace::marketplace::Rol::Comprador)
                .collect();

            // Ordenar por reputación descendente
            compradores.sort_by(|a, b| {
                b.get_reputacion_como_comprador()
                    .cmp(&a.get_reputacion_como_comprador())
            });

            // Tomar los primeros 5 (o menos)
            compradores.into_iter().take(5).collect()
        }

        // #[ink(message)]
        // pub fn get_productos_mas_vendido(&self) -> Producto {
            
        // }

        /// Retorna las estadísticas por categoría.
        ///
        /// Filtra las ordenes y cuenta cuántas ventas hay por categoría.
        ///
        /// # Retorna
        /// - Un `Vec<EstadisticaCategoria>` con la cantidad de ventas por categoría.
        #[ink(message)]
        pub fn get_estadistica_por_categoria(&self) -> Vec<EstadisticaCategoria> {
            let lista_ordenes = self.marketplace.get_ordenes();
            let mut estadisticas: Vec<EstadisticaCategoria> = Vec::new();
            for orden in lista_ordenes {
                if let Some(estadistica) = estadisticas.iter_mut().find(|c| c.categoria == orden.get_publicacion().get_categoria()) {
                    estadistica.total_ventas += orden.get_cantidad();
                } else {
                    estadisticas.push(EstadisticaCategoria {
                        categoria: orden.get_publicacion().get_categoria(),
                        total_ventas: orden.get_cantidad(),
                        //calificacion_promedio: orden.get_producto().get_calificacion_promedio(),
                    });
                }
            }
            estadisticas
        }

        /// Retorna la cantidad de ordenes por usuario.
        /// 
        /// Filtra las ordenes y cuenta cuántas ordenes tiene cada usuario.
        ///
        /// # Retorna
        /// - Un `Vec<EstadisticaUsuario>` con la cantidad de ordenes por usuario.
        #[ink(message)]
        pub fn get_cant_ordenes_por_usuario(&self) -> Vec<EstadisticaUsuario> {
            let lista_ordenes = self.marketplace.get_ordenes();
            let mut estadisticas: Vec<EstadisticaUsuario> = Vec::new();
            for orden in lista_ordenes {
                if let Some(estadistica) = estadisticas.iter_mut().find(|u| u.get_usuario_id() == orden.get_comprador()) {
                    estadistica.cant_ordenes = estadistica.cant_ordenes.saturating_add(1);
                } else {
                    estadisticas.push(EstadisticaUsuario {
                        usuario: self.marketplace.get_usuario_by_id(orden.get_comprador()).unwrap(),
                        cant_ordenes: 1,
                    });
                }    
            }
            estadisticas
        }
    }

    #[cfg(test)]
    mod tests {
        
    }

}
