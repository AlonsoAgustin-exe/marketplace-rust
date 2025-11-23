#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod marketplace {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Marketplace {
        /// storage de usuarios
        usuarios: Mapping<AccountId, Usuario>, // (id_usuario, datos_usuario)

        /// storage general de publicaciones y ordenes de compra
        publicaciones: Vec<Publicacion>,
        ordenes_compra: Vec<OrdenCompra>,

        /// storage mapping de publicaciones por vendedor
        publicaciones_mapping: Mapping<AccountId, Vec<u32>>, // (id_vendedor, id's publicaciones)
        /// storage mapping de ordenes de compra por comprador
        ordenes_compra_mapping: Mapping<AccountId, Vec<u32>>, // (id_comprador, id's ordenes de compra)
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, PartialEq)]
    /// Define los posibles errores del sistema que pueden ocurrir
    /// durante la ejecución del contrato.
    pub enum ErrorSistema {
        /// El usuario no está registrado en el sistema.
        UsuarioNoRegistrado,

        /// El usuario ya se encuentra registrado.
        UsuarioYaRegistrado,

        /// El usuario no posee permisos de vendedor.
        UsuarioNoEsVendedor,

        /// El usuario no posee permisos de comprador.
        UsuarioNoEsComprador,

        /// El vendedor especificado no existe.
        VendedorNoExistente,

        /// El vendedor no tiene publicaciones registradas.
        VendedorSinPublicaciones,

        /// La publicación no posee stock disponible.
        PublicacionSinStock,

        /// La publicación solicitada no existe.
        PublicacionNoExistente,

        /// Error por desbordamiento negativo al manipular publicaciones.
        UnderflowPublicaciones,

        /// Error por desbordamiento negativo al manipular órdenes.
        UnderflowOrdenes,

        /// El usuario que intenta realizar la acción no es el vendedor asociado a la orden.
        NoEresVendedorDeLaOrden,

        /// El usuario que intenta realizar la acción no es el comprador asociado a la orden.
        NoEresCompradorDeLaOrden,

        /// La orden ya ha sido marcada como enviada previamente.
        YaEnviada,

        /// La orden ya ha sido marcada como recibida previamente.
        YaRecibido,

        /// La orden ha sido cancelada y no puede ser modificada.
        OrdenCancelada,

        /// La orden aún está pendiente y no puede ser marcada como recibida.
        OrdenPendiente,

        /// No hay petición de cancelación activa para esta orden.
        PeticionNoSolicitada,

        /// La orden no se encuentra en estado pendiente.
        OrdenNoPendiente,

        /// El usuario no tiene permisos para realizar la acción.
        SinPermisos,

        /// Error por desbordamiento positivo al manipular publicaciones.
        OverflowPublicaciones,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, Clone, PartialEq)]
    /// Representa un usuario registrado dentro del sistema.
    pub struct Usuario {
        /// Nombre de usuario asociado a la cuenta.
        username: String,

        /// Rol asignado al usuario dentro del sistema.
        rol: Rol,

        /// Identificador único de la cuenta en la red.
        account_id: AccountId,
    }


    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, Clone, PartialEq)]
    /// Define los posibles roles que un usuario puede tener dentro del sistema.
    pub enum Rol {
        /// Usuario con permisos para realizar compras.
        Comprador,

        /// Usuario con permisos para publicar y vender productos.
        Vendedor,

        /// Usuario con permisos tanto de comprador como de vendedor.
        Ambos,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, Clone, PartialEq)]
    /// Representa una publicación disponible dentro del sistema.
    pub struct Publicacion {
        /// Identificador único de la publicación.
        id_publicacion: u64,

        /// Nombre del producto o ítem publicado.
        nombre: String,

        /// Descripción del producto.
        descripcion: String,

        /// Precio del producto en la unidad base del token.
        precio: u64,

        /// Categoría a la que pertenece el producto.
        categoria: Categoria,

        /// Cantidad disponible en stock.
        stock: u64,

        /// Identificador de cuenta del vendedor asociado.
        vendedor_id: AccountId,
    }


    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, Clone, PartialEq)]
    /// Define las categorías disponibles para las publicaciones dentro del sistema.
    pub enum Categoria {
        /// Productos relacionados con equipos y componentes de computación.
        Computacion,

        /// Prendas de vestir y accesorios.
        Ropa,

        /// Herramientas manuales o eléctricas.
        Herramientas,

        /// Muebles y artículos para el hogar.
        Muebles,
    }


    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, Clone, PartialEq)]
    /// Representa una orden de compra dentro del sistema.
    pub struct OrdenCompra {
        /// Estado actual de la orden.
        estado: Estado,

        /// Publicación asociada a la orden.
        publicacion: Publicacion,

        /// Identificador de cuenta del comprador que realizó la orden.
        comprador_id: AccountId,

        /// Indica si se ha solicitado la cancelación de la orden.
        peticion_cancelacion: bool,

        /// Cantidad de productos comprados.
        cantidad: u32,
    }


    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, Clone, PartialEq)]
    /// Define los posibles estados de una orden de compra.
    pub enum Estado {
        /// La orden ha sido creada pero aún no procesada.
        Pendiente,

        /// La orden ha sido enviada al comprador.
        Enviada,

        /// La orden ha sido recibida por el comprador.
        Recibida,

        /// La orden ha sido cancelada.
        Cancelada,
    }


    impl Marketplace {
        /// Constructor del contrato `Marketplace`.
        ///
        /// Inicializa el contrato con colecciones vacías para usuarios,
        /// publicaciones, órdenes de compra y sus mapeos asociados.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                usuarios: Default::default(),
                publicaciones: Default::default(),
                ordenes_compra: Default::default(),
                publicaciones_mapping: Default::default(),
                ordenes_compra_mapping: Default::default(),
            }
        }

        /// Registra un nuevo usuario en el sistema.
        ///
        /// Delega la creación al método interno `_registrar_usuario`.
        ///
        /// # Parámetros
        /// - `username`: Nombre de usuario a registrar.
        /// - `rol`: Rol asignado al usuario.
        ///
        /// # Retorna
        /// - `Ok(Usuario)` si el registro se realizó correctamente.
        /// - `Err(ErrorSistema::UsuarioYaRegistrado)` si el usuario ya existía.
        #[ink(message)]
        #[ignore]
        pub fn registrar_usuario(&mut self,username: String,rol: Rol,) -> Result<Usuario, ErrorSistema> {
            self._registrar_usuario(self.env().caller(), username, rol)
        }

        /// Método interno que realiza la lógica de registro de un usuario.
        ///
        /// # Parámetros
        /// - `caller`: Identificador de la cuenta que realiza el registro.
        /// - `username`: Nombre de usuario a registrar.
        /// - `rol`: Rol asignado al usuario.
        ///
        /// # Retorna
        /// - `Ok(Usuario)` si el registro se realizó correctamente.
        /// - `Err(ErrorSistema::UsuarioYaRegistrado)` si el usuario ya existía.
        ///
        /// Nota: Este método es auxiliar y no se expone como mensaje del contrato.
        fn _registrar_usuario( &mut self,caller: AccountId,username: String,rol: Rol,) -> Result<Usuario, ErrorSistema> {
            //Verifica si el usuario ya esta registrado
            if self.usuarios.get(caller).is_some() {
                return Err(ErrorSistema::UsuarioYaRegistrado);
            };

            //Crea el nuevo usuario
            let usuario = Usuario {
                account_id: caller,
                username,
                rol,
            };

            //Almacena el nuevo usuario en el sistema
            self.usuarios.insert(caller, &usuario);

            //Retorna el usuario creado
            Ok(usuario)
        }

        /// Obtiene la información del usuario que llama al contrato.
        ///
        /// Delegará la obtención al método interno `_get_usuario`.
        ///
        /// # Retorna
        /// - `Ok(Usuario)` con los datos del usuario.
        /// - `Err(ErrorSistema::UsuarioNoRegistrado)` si el usuario no está registrado.
        #[ink(message)]
        #[ignore]
        pub fn get_usuario(&self) -> Result<Usuario, ErrorSistema> {
            self._get_usuario(self.env().caller())
        }

        /// Método interno que obtiene la información de un usuario específico.
        ///
        /// # Parámetros
        /// - `caller`: Identificador de la cuenta del usuario a consultar.
        ///
        /// # Retorna
        /// - `Ok(Usuario)` con los datos del usuario.
        /// - `Err(ErrorSistema::UsuarioNoRegistrado)` si el usuario no está registrado.
        ///
        /// Nota: Este método es auxiliar y no se expone como mensaje del contrato.
        fn _get_usuario(&self, caller: AccountId) -> Result<Usuario, ErrorSistema> {
            self.usuarios.get(caller).ok_or(ErrorSistema::UsuarioNoRegistrado)
        }

        /// Cambia el rol del usuario que llama al contrato.
        ///
        /// Delegará la modificación al método interno `_cambiar_rol`.
        ///
        /// # Parámetros
        /// - `nuevo_rol`: Nuevo rol que se asignará al usuario.
        ///
        /// # Retorna
        /// - `Ok(Usuario)` con los datos actualizados.
        /// - `Err(ErrorSistema::UsuarioNoRegistrado)` si el usuario no está registrado.
        #[ink(message)]
        #[ignore]
        pub fn cambiar_rol(&mut self, nuevo_rol: Rol) -> Result<Usuario, ErrorSistema> {
            self._cambiar_rol(nuevo_rol)
        }

        /// Método interno que realiza la lógica de cambio de rol de un usuario.
        ///
        /// # Parámetros
        /// - `caller`: Identificador de la cuenta del usuario.
        /// - `nuevo_rol`: Nuevo rol a asignar.
        ///
        /// # Retorna
        /// - `Ok(Usuario)` con los datos actualizados.
        /// - `Err(ErrorSistema::UsuarioNoRegistrado)` si el usuario no está registrado.
        ///
        /// Nota: Este método es auxiliar y no se expone como mensaje del contrato.
        fn _cambiar_rol(&mut self, nuevo_rol: Rol) -> Result<Usuario, ErrorSistema> {
            let mut usuario = self.get_usuario()?;
            usuario.rol = nuevo_rol;
            self.usuarios.insert(usuario.account_id, &usuario);
            Ok(usuario)
        }

        /// Publica un nuevo producto en el marketplace para el usuario que llama al contrato.
        ///
        /// Delegará la creación y almacenamiento al método interno `_publicar`.
        ///
        /// # Parámetros
        /// - `nombre`: Nombre del producto.
        /// - `descripcion`: Descripción del producto.
        /// - `precio`: Precio del producto en la unidad base del token.
        /// - `categoria`: Categoría a la que pertenece el producto.
        /// - `stock`: Cantidad disponible del producto.
        ///
        /// # Retorna
        /// - `Ok(Publicacion)` con los datos de la nueva publicación.
        /// - `Err(ErrorSistema)` si ocurre algún error durante el registro.
        #[ink(message)]
        #[ignore]
        pub fn publicar(
            &mut self,
            nombre: String,
            descripcion: String,
            precio: u64,
            categoria: Categoria,
            stock: u64,
        ) -> Result<Publicacion, ErrorSistema> {
            self._publicar(
                self.env().caller(),
                nombre,
                descripcion,
                precio,
                categoria,
                stock,
            )
        }

        
        /// Método interno que realiza la lógica de creación y almacenamiento de una publicación.
        ///
        /// # Parámetros
        /// - `caller`: Identificador de la cuenta del usuario que publica.
        /// - `nombre`: Nombre del producto.
        /// - `descripcion`: Descripción del producto.
        /// - `precio`: Precio del producto en la unidad base del token.
        /// - `categoria`: Categoría a la que pertenece el producto.
        /// - `stock`: Cantidad disponible del producto.
        ///
        /// # Retorna
        /// - `Ok(Publicacion)` con los datos de la publicación creada.
        /// - `Err(ErrorSistema)` si el usuario no es vendedor, no está registrado o hay errores de indexación.
        ///
        /// Nota: Este método es auxiliar y no se expone como mensaje del contrato.
        fn _publicar(
            &mut self,
            caller: AccountId,
            nombre: String,
            descripcion: String,
            precio: u64,
            categoria: Categoria,
            stock: u64,
        ) -> Result<Publicacion, ErrorSistema> {
            //Validacion de usuario
            let usuario = self._get_usuario(caller)?;
            usuario.es_vendedor()?;

            //Crea la publicacion
            let publicacion = Publicacion::new(
                self.publicaciones.len() as u64,
                nombre,
                descripcion,
                precio,
                categoria,
                stock,
                usuario.account_id,
            );

            //Agrega la publicacion al sistema
            self.publicaciones.push(publicacion.clone());
            //Agrega el index de la publicacion al vector personal del vendedor
            let mut publicaciones_vendedor = self
                .publicaciones_mapping
                .get(usuario.account_id)
                .unwrap_or_default();

            let index_pub = (self.publicaciones.len() as u32)
                .checked_sub(1)
                .ok_or(ErrorSistema::UnderflowPublicaciones)?; // Calcula el index
            publicaciones_vendedor.push(index_pub); // Agrega el index de la publicacion

            //Almacena el vector de indexs del usuario
            self.publicaciones_mapping
                .insert(usuario.account_id, &publicaciones_vendedor);

            Ok(publicacion)
        }

        /// Retorna las publicaciones del vendedor solicitante.
        ///
        /// Delegará la obtención al método interno `_get_publicaciones_vendedor`.
        ///
        /// # Retorna
        /// - `Ok(Vec<Publicacion>)` con la lista de publicaciones del vendedor.
        /// - `Err(ErrorSistema)` si el usuario no es vendedor o no está registrado.
        #[ink(message)]
        #[ignore]
        pub fn get_publicaciones_vendedor(&self) -> Result<Vec<Publicacion>, ErrorSistema> {
            self._get_publicaciones_vendedor(self.env().caller())
        }

        /// Método interno que obtiene las publicaciones de un vendedor específico.
        ///
        /// # Parámetros
        /// - `caller`: Identificador de la cuenta del vendedor.
        ///
        /// # Retorna
        /// - `Ok(Vec<Publicacion>)` con la lista de publicaciones.
        /// - `Err(ErrorSistema)` si el usuario no es vendedor o no está registrado.
        ///
        /// Nota: Este método es auxiliar y no se expone como mensaje del contrato.
        fn _get_publicaciones_vendedor(
            &self,
            caller: AccountId,
        ) -> Result<Vec<Publicacion>, ErrorSistema> {
            //Validacion de usuario
            let usuario = self._get_usuario(caller)?;
            usuario.es_vendedor()?;

            //Obtiene el vector con ids de publicaciones del vendedor
            let ids_publicaciones_vendedor = self
                .publicaciones_mapping
                .get(usuario.account_id)
                .unwrap_or_default();

            //Recorre las publicaciones del sistema y arma un vector con las
            //publicaciones del vendedor solicitante
            let publicaciones_vendedor = ids_publicaciones_vendedor
                .iter()
                .filter_map(|&i| self.publicaciones.get(i as usize))
                .cloned()
                .collect();

            Ok(publicaciones_vendedor)
        }

        /// Retorna todas las publicaciones existentes en el sistema.
        ///
        /// Delegará la obtención al método interno `_get_publicaciones`.
        ///
        /// # Retorna
        /// - `Ok(Vec<Publicacion>)` con la lista completa de publicaciones.
        /// - `Err(ErrorSistema)` si el usuario solicitante no está registrado.
        #[ink(message)]
        #[ignore]
        pub fn get_publicaciones(&self) -> Result<Vec<Publicacion>, ErrorSistema> {
            self._get_publicaciones(self.env().caller())
        }

        /// Método interno que obtiene todas las publicaciones.
        ///
        /// # Parámetros
        /// - `caller`: Identificador de la cuenta que realiza la consulta.
        ///
        /// # Retorna
        /// - `Ok(Vec<Publicacion>)` con la lista completa de publicaciones.
        /// - `Err(ErrorSistema)` si el usuario no está registrado.
        ///
        /// Nota: Este método es auxiliar y no se expone como mensaje del contrato.
        fn _get_publicaciones(&self, caller: AccountId) -> Result<Vec<Publicacion>, ErrorSistema> {
            self._get_usuario(caller)?;
            Ok(self.publicaciones.clone())
        }

        /// Crea una nueva orden de compra para una publicación específica.
        ///
        /// Delegará la creación al método interno `_ordenar_compra`.
        ///
        /// # Parámetros
        /// - `idx_publicacion`: Índice de la publicación a comprar.
        /// - `cantidad`: Cantidad de unidades a comprar.
        ///
        /// # Retorna
        /// - `Ok(OrdenCompra)` con los detalles de la orden creada.
        /// - `Err(ErrorSistema)` si ocurre algún error (ej. sin stock, usuario no comprador).
        #[ink(message)]
        #[ignore]
        pub fn ordenar_compra(
            &mut self,
            idx_publicacion: u32,
            cantidad: u32,
        ) -> Result<OrdenCompra, ErrorSistema> {
            self._ordenar_compra(self.env().caller(), idx_publicacion, cantidad)
        }

        /// Método interno que realiza la lógica de creación de una orden de compra.
        ///
        /// # Parámetros
        /// - `caller`: Identificador de la cuenta del comprador.
        /// - `idx_publicacion`: Índice de la publicación.
        /// - `cantidad`: Cantidad de unidades a comprar.
        ///
        /// # Retorna
        /// - `Ok(OrdenCompra)` con los detalles de la orden.
        /// - `Err(ErrorSistema)` si el usuario no es comprador, la publicación no existe o no hay stock.
        ///
        /// Nota: Este método es auxiliar y no se expone como mensaje del contrato.
        fn _ordenar_compra(
            &mut self,
            caller: AccountId,
            idx_publicacion: u32,
            cantidad: u32,
        ) -> Result<OrdenCompra, ErrorSistema> {
            // validaciones de usuario
            let usuario = self._get_usuario(caller)?;
            usuario.es_comprador()?;

            //Buscar publicacion
            let mut publicacion = self
                .publicaciones
                .get(idx_publicacion as usize)
                .cloned()
                .ok_or(ErrorSistema::PublicacionNoExistente)?;

            //Decrementar Stock
            publicacion.stock = publicacion
                .stock
                .checked_sub(cantidad as u64)
                .ok_or(ErrorSistema::PublicacionSinStock)?;

            // Reemplazar la publicación modificada
            self.publicaciones[idx_publicacion as usize] = publicacion.clone();

            // crear orden de compra
            let orden_compra = OrdenCompra {
                estado: Estado::Pendiente,
                publicacion: publicacion.clone(),
                comprador_id: usuario.account_id,
                peticion_cancelacion: false,
                cantidad,
            };

            //Agrega la orden de compra al sistema
            self.ordenes_compra.push(orden_compra.clone());
            //Agrega el index de la orden de compra al vector personal del comprador
            let mut ordenes_compra_comprador = self
                .ordenes_compra_mapping
                .get(usuario.account_id)
                .unwrap_or_default();

            let index_ord = (self.ordenes_compra.len() as u32)
                .checked_sub(1)
                .ok_or(ErrorSistema::UnderflowOrdenes)?; // Calcula el index
            ordenes_compra_comprador.push(index_ord); // Agrega el index de la orden de compra

            //Almacena el vector de indexs del usuario
            self.ordenes_compra_mapping
                .insert(usuario.account_id, &ordenes_compra_comprador);

            Ok(orden_compra)
        }

        /// Retorna las órdenes de compra del comprador solicitante.
        ///
        /// Delegará la obtención al método interno `_get_ordenes_comprador`.
        ///
        /// # Retorna
        /// - `Ok(Vec<OrdenCompra>)` con la lista de órdenes del comprador.
        /// - `Err(ErrorSistema)` si el usuario no es comprador o no está registrado.
        #[ink(message)]
        #[ignore]
        pub fn get_ordenes_comprador(&self) -> Result<Vec<OrdenCompra>, ErrorSistema> {
            self._get_ordenes_comprador(self.env().caller())
        }

        /// Método interno que obtiene las órdenes de compra de un comprador específico.
        ///
        /// # Parámetros
        /// - `caller`: Identificador de la cuenta del comprador.
        ///
        /// # Retorna
        /// - `Ok(Vec<OrdenCompra>)` con la lista de órdenes.
        /// - `Err(ErrorSistema)` si el usuario no es comprador o no está registrado.
        ///
        /// Nota: Este método es auxiliar y no se expone como mensaje del contrato.
        fn _get_ordenes_comprador(
            &self,
            caller: AccountId,
        ) -> Result<Vec<OrdenCompra>, ErrorSistema> {
            //Validacion de usuario
            let usuario = self._get_usuario(caller)?;
            usuario.es_comprador()?;

            //Obtiene el vector con ids de ordenes de compra del comprador
            let ids_ordenes_compra_comprador = self
                .ordenes_compra_mapping
                .get(usuario.account_id)
                .unwrap_or_default();

            //Recorre las ordenes de compra del sistema y arma un vector con las
            //ordenes de compra del comprador solicitante
            let ordenes_compra_comprador = ids_ordenes_compra_comprador
                .iter()
                .filter_map(|&i| self.ordenes_compra.get(i as usize))
                .cloned()
                .collect();

            Ok(ordenes_compra_comprador)
        }

        /// Retorna todas las órdenes de compra existentes en el sistema.
        ///
        /// Delegará la obtención al método interno `_get_ordenes`.
        ///
        /// # Retorna
        /// - `Ok(Vec<OrdenCompra>)` con la lista completa de órdenes.
        /// - `Err(ErrorSistema)` si el usuario solicitante no está registrado.
        #[ink(message)]
        #[ignore]
        pub fn get_ordenes(&self) -> Result<Vec<OrdenCompra>, ErrorSistema> {
            self._get_ordenes(self.env().caller())
        }

        /// Método interno que obtiene todas las órdenes de compra.
        ///
        /// # Parámetros
        /// - `caller`: Identificador de la cuenta que realiza la consulta.
        ///
        /// # Retorna
        /// - `Ok(Vec<OrdenCompra>)` con la lista completa de órdenes.
        /// - `Err(ErrorSistema)` si el usuario no está registrado.
        ///
        /// Nota: Este método es auxiliar y no se expone como mensaje del contrato.
        fn _get_ordenes(&self, caller: AccountId) -> Result<Vec<OrdenCompra>, ErrorSistema> {
            self._get_usuario(caller)?;
            Ok(self.ordenes_compra.clone())
        }

        /// Marca una orden de compra como enviada.
        ///
        /// Solo el vendedor asociado a la orden puede realizar esta acción.
        ///
        /// # Parámetros
        /// - `idx_orden`: Índice de la orden a marcar.
        ///
        /// # Retorna
        /// - `Ok(OrdenCompra)` con el estado actualizado a `Enviada`.
        /// - `Err(ErrorSistema)` si ocurre algún error (ej. no es el vendedor, estado incorrecto).
        #[ink(message)]
        pub fn marcar_enviado(&mut self, idx_orden: u32) -> Result<OrdenCompra, ErrorSistema> {
            self._marcar_enviado(self.env().caller(), idx_orden)
        }

        /// Método interno que realiza la lógica para marcar una orden como enviada.
        ///
        /// # Parámetros
        /// - `caller`: Identificador de la cuenta del vendedor.
        /// - `idx_orden`: Índice de la orden.
        ///
        /// # Retorna
        /// - `Ok(OrdenCompra)` con el estado actualizado.
        /// - `Err(ErrorSistema)` si el usuario no es vendedor, no es el dueño de la orden o el estado no es `Pendiente`.
        fn _marcar_enviado(&mut self, caller: AccountId, idx_orden: u32) -> Result<OrdenCompra, ErrorSistema> {
            // valida la existencia y rol del usuario
            let usuario = self._get_usuario(caller)?;
            usuario.es_vendedor()?;

            //Buscar orden
            let orden = self
                .ordenes_compra
                .get_mut(idx_orden as usize)
                .ok_or(ErrorSistema::PublicacionNoExistente)?;

            match orden.estado {
                Estado::Pendiente => {
                    //Verifica que el vendedor sea el de la orden
                    if orden.publicacion.vendedor_id != usuario.account_id {
                        return Err(ErrorSistema::NoEresVendedorDeLaOrden);
                    }
                    //Marca la orden como enviada
                    orden.estado = Estado::Enviada;
                    Ok(orden.clone())
                }
                Estado::Enviada => Err(ErrorSistema::YaEnviada),
                Estado::Recibida => Err(ErrorSistema::YaRecibido),
                Estado::Cancelada => Err(ErrorSistema::OrdenCancelada),
            }
        }

        /// Marca una orden de compra como recibida.
        ///
        /// Solo el comprador asociado a la orden puede realizar esta acción.
        ///
        /// # Parámetros
        /// - `idx_orden`: Índice de la orden a marcar.
        ///
        /// # Retorna
        /// - `Ok(OrdenCompra)` con el estado actualizado a `Recibida`.
        /// - `Err(ErrorSistema)` si ocurre algún error (ej. no es el comprador, estado incorrecto).
        #[ink(message)]
        pub fn marcar_recibido(&mut self, idx_orden: u32) -> Result<OrdenCompra, ErrorSistema> {
            self._marcar_recibido(self.env().caller(), idx_orden)
        }

        /// Método interno que realiza la lógica para marcar una orden como recibida.
        ///
        /// # Parámetros
        /// - `caller`: Identificador de la cuenta del comprador.
        /// - `idx_orden`: Índice de la orden.
        ///
        /// # Retorna
        /// - `Ok(OrdenCompra)` con el estado actualizado.
        /// - `Err(ErrorSistema)` si el usuario no es comprador, no es el dueño de la orden o el estado no es `Enviada`.
        fn _marcar_recibido(&mut self, caller: AccountId, idx_orden: u32,) -> Result<OrdenCompra, ErrorSistema> {
            // valida la existencia y rol del usuario
            let usuario = self._get_usuario(caller)?;
            usuario.es_comprador()?;

            //Buscar orden
            let orden = self
                .ordenes_compra
                .get_mut(idx_orden as usize)
                .ok_or(ErrorSistema::PublicacionNoExistente)?;

            match orden.estado {
                Estado::Enviada => {
                    //Verifica que el comprador sea el de la orden
                    if orden.comprador_id != usuario.account_id {
                        return Err(ErrorSistema::NoEresCompradorDeLaOrden);
                    }
                    //Marca la orden como recibida
                    orden.estado = Estado::Recibida;
                    Ok(orden.clone())
                }
                Estado::Pendiente => Err(ErrorSistema::OrdenPendiente),
                Estado::Recibida => Err(ErrorSistema::YaRecibido),
                Estado::Cancelada => Err(ErrorSistema::OrdenCancelada),
            }

        }

        /// Cancela una orden de compra.
        ///
        /// Este método permite iniciar el proceso de cancelación de una orden.
        /// Requiere que el comprador solicite la cancelación y luego el vendedor la apruebe.
        ///
        /// # Parámetros
        /// - `idx_orden`: Índice de la orden a cancelar.
        ///
        /// # Retorna
        /// - `Ok(OrdenCompra)` con el estado actualizado de la orden.
        /// - `Err(ErrorSistema)` si ocurre algún error (ej. orden no encontrada, usuario no autorizado).
        #[ink(message)]
        pub fn cancelar_orden(&mut self, idx_orden: u32) -> Result<OrdenCompra, ErrorSistema> {
            self._cancelar_orden(self.env().caller(), idx_orden)
        }

        /// Método interno que maneja la lógica de cancelación de órdenes.
        ///
        /// # Parámetros
        /// - `caller`: Identificador de la cuenta que solicita la acción.
        /// - `idx_orden`: Índice de la orden a cancelar.
        ///
        /// # Retorna
        /// - `Ok(OrdenCompra)` con el estado actualizado.
        /// - `Err(ErrorSistema)` en caso de error.
        ///
        /// # Lógica
        /// - Si el `caller` es el comprador: Se marca `peticion_cancelacion` como `true`.
        /// - Si el `caller` es el vendedor: Se verifica que exista una petición, se restaura el stock y se cambia el estado a `Cancelada`.
        /// - Si el `caller` no es ninguno de los dos: Retorna `ErrorSistema::SinPermisos`.
        fn _cancelar_orden(&mut self, caller: AccountId, idx_orden: u32) -> Result<OrdenCompra, ErrorSistema> {
            // Validar usuario
            self._get_usuario(caller)?;

            // Buscar orden
            let orden = self
                .ordenes_compra
                .get_mut(idx_orden as usize)
                .ok_or(ErrorSistema::PublicacionNoExistente)?;

            // Verificar estado
            if orden.estado != Estado::Pendiente {
                return Err(ErrorSistema::OrdenNoPendiente);
            }

            // Lógica según rol
            if caller == orden.comprador_id {
                // Comprador solicita cancelación
                orden.peticion_cancelacion = true;
                Ok(orden.clone())
            } else if caller == orden.publicacion.vendedor_id {
                // Vendedor aprueba cancelación
                if !orden.peticion_cancelacion {
                    return Err(ErrorSistema::PeticionNoSolicitada);
                }

                // Restaurar stock
                let mut publicacion = self
                    .publicaciones
                    .get_mut(orden.publicacion.id_publicacion as usize)
                    .ok_or(ErrorSistema::PublicacionNoExistente)?;
                
                publicacion.stock = publicacion.stock.checked_add(orden.cantidad as u64).ok_or(ErrorSistema::OverflowPublicaciones)?;
                
                // Actualizar estado orden
                orden.estado = Estado::Cancelada;
                
                Ok(orden.clone())
            } else {
                // Ni comprador ni vendedor
                Err(ErrorSistema::SinPermisos)
            }
        }
    }

    impl Publicacion {
        /// Crea una nueva instancia de `Publicacion`.
        ///
        /// # Parámetros
        /// - `id_publicacion`: Identificador único de la publicación.
        /// - `nombre`: Nombre del producto.
        /// - `descripcion`: Descripción del producto.
        /// - `precio`: Precio del producto.
        /// - `categoria`: Categoría del producto.
        /// - `stock`: Cantidad disponible.
        /// - `vendedor_id`: Identificador del vendedor.
        ///
        /// # Retorna
        /// - Una nueva instancia de `Publicacion`.
        pub fn new(
            id_publicacion: u64,
            nombre: String,
            descripcion: String,
            precio: u64,
            categoria: Categoria,
            stock: u64,
            vendedor_id: AccountId,
        ) -> Publicacion {
            Publicacion {
                id_publicacion,
                nombre,
                descripcion,
                precio,
                categoria,
                stock,
                vendedor_id,
            }
        }
    }

    impl Usuario {
        /// Valida que el usuario tenga rol `Vendedor` o `Ambos`.
        ///
        /// # Retorna
        /// - `Ok(true)` si el usuario tiene permisos de vendedor.
        /// - `Err(ErrorSistema::UsuarioNoEsVendedor)` si el usuario es solo comprador.
        fn es_vendedor(&self) -> Result<bool, ErrorSistema> {
            if matches!(self.rol, Rol::Comprador) {
                Err(ErrorSistema::UsuarioNoEsVendedor)
            } else {
                Ok(true)
            }
        }

        /// Valida que el usuario tenga rol `Comprador` o `Ambos`.
        ///
        /// # Retorna
        /// - `Ok(true)` si el usuario tiene permisos de comprador.
        /// - `Err(ErrorSistema::UsuarioNoEsComprador)` si el usuario es solo vendedor.
        fn es_comprador(&self) -> Result<bool, ErrorSistema> {
            if matches!(self.rol, Rol::Vendedor) {
                Err(ErrorSistema::UsuarioNoEsComprador)
            } else {
                Ok(true)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        mod tests_es_vendedor {
            use super::*;

            /// Verifica que un usuario con rol `Vendedor` sea identificado correctamente como vendedor.
            #[test]
            fn tests_es_vendedor_true_vendedor() {
                let usuario = Usuario {
                    account_id: AccountId::from([0xAA; 32]),
                    username: "agustin22".to_string(),
                    rol: Rol::Vendedor,
                };

                assert_eq!(usuario.es_vendedor().is_ok(), true);
            }

            /// Verifica que un usuario con rol `Ambos` sea identificado correctamente como vendedor.
            #[test]
            fn tests_es_vendedor_true_ambos() {
                let usuario = Usuario {
                    account_id: AccountId::from([0xAA; 32]),
                    username: "agustin22".to_string(),
                    rol: Rol::Ambos,
                };

                assert_eq!(usuario.es_vendedor().is_ok(), true);
            }

            /// Verifica que un usuario con rol `Comprador` NO sea identificado como vendedor.
            #[test]
            fn tests_es_vendedor_false() {
                let usuario = Usuario {
                    account_id: AccountId::from([0xAA; 32]),
                    username: "agustin22".to_string(),
                    rol: Rol::Comprador,
                };

                assert_eq!(usuario.es_vendedor().is_ok(), false);
            }
        }

        mod tests_es_comprador {
            use super::*;

            /// Verifica que un usuario con rol `Comprador` sea identificado correctamente como comprador.
            #[test]
            fn tests_es_comprador_true_comprador() {
                let usuario = Usuario {
                    account_id: AccountId::from([0xAA; 32]),
                    username: "agustin22".to_string(),
                    rol: Rol::Comprador,
                };

                assert_eq!(usuario.es_comprador().is_ok(), true);
            }

            /// Verifica que un usuario con rol `Ambos` sea identificado correctamente como comprador.
            #[test]
            fn tests_es_comprador_true_ambos() {
                let usuario = Usuario {
                    account_id: AccountId::from([0xAA; 32]),
                    username: "agustin22".to_string(),
                    rol: Rol::Ambos,
                };

                assert_eq!(usuario.es_comprador().is_ok(), true);
            }

            /// Verifica que un usuario con rol `Vendedor` NO sea identificado como comprador.
            #[test]
            fn tests_es_comprador_false() {
                let usuario = Usuario {
                    account_id: AccountId::from([0xAA; 32]),
                    username: "agustin22".to_string(),
                    rol: Rol::Vendedor,
                };

                assert_eq!(usuario.es_comprador().is_ok(), false);
            }
        }

        mod tests_registrar_usuario {
            use super::*;

            /// Verifica que un usuario nuevo pueda registrarse correctamente.
            #[ink::test]
            fn tests_registrar_usuario_no_registrado() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);
                let username = "agustin".to_string();
                let rol = Rol::Ambos;

                assert_eq!(marketplace._registrar_usuario(caller, username, rol).is_ok(),true);
            }

            /// Verifica que no se pueda registrar un usuario que ya existe.
            #[ink::test]
            fn tests_registrar_usuario_ya_registrado_error() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);
                let username = "agustin".to_string();
                let rol = Rol::Ambos;

                assert_eq!(
                    marketplace
                        ._registrar_usuario(caller.clone(), username.clone(), rol.clone())
                        .is_ok(),
                    true
                );

                let result = marketplace._registrar_usuario(caller, username, rol);

                assert_eq!(result, Err(ErrorSistema::UsuarioYaRegistrado));
            }
        }

        mod tests_get_usuario {
            use super::*;

            /// Verifica que se pueda obtener la información de un usuario registrado.
            #[ink::test]
            fn tests_get_usuario_encontrado() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);
                let username = "agustin".to_string();
                let rol = Rol::Ambos;

                let _ = marketplace._registrar_usuario(caller, username, rol);

                assert_eq!(marketplace._get_usuario(caller).is_ok(), true);
            }

            /// Verifica que se retorne un error al intentar obtener un usuario no registrado.
            #[ink::test]
            fn tests_get_usuario_no_encontrado() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);

                let result = marketplace._get_usuario(caller);

                assert_eq!(result, Err(ErrorSistema::UsuarioNoRegistrado));
            }
        }

        mod tests_cambiar_rol {
            use super::*;

            /// Verifica que un usuario pueda cambiar su rol de `Comprador` a `Vendedor`.
            #[ink::test]
            fn tests_cambiar_rol_comprador_a_vendedor() {
                let mut marketplace = Marketplace::new();
                let caller = AccountId::from([0xAA; 32]);

                let _ = marketplace.registrar_usuario("agustin".to_string(), Rol::Comprador);
                let result = marketplace.cambiar_rol(Rol::Vendedor);

                assert!(result.is_ok());
                if let Ok(usuario) = result {
                    assert_eq!(usuario.rol, Rol::Vendedor);
                }
            }

            /// Verifica que un usuario pueda cambiar su rol de `Vendedor` a `Comprador`.
            #[ink::test]
            fn tests_cambiar_rol_vendedor_a_comprador() {
                let mut marketplace = Marketplace::new();
                let caller = AccountId::from([0xAA; 32]);

                let _ = marketplace.registrar_usuario("agustin".to_string(), Rol::Vendedor);
                let result = marketplace.cambiar_rol(Rol::Comprador);

                assert!(result.is_ok());
                if let Ok(usuario) = result {
                    assert_eq!(usuario.rol, Rol::Comprador);
                }
            }

            /// Verifica que un usuario pueda cambiar su rol a `Ambos`.
            #[ink::test]
            fn tests_cambiar_rol_a_ambos() {
                let mut marketplace = Marketplace::new();
                let caller = AccountId::from([0xAA; 32]);

                let _ = marketplace.registrar_usuario("test".to_string(), Rol::Comprador);
                let result = marketplace.cambiar_rol(Rol::Ambos);

                assert!(result.is_ok());
                if let Ok(usuario) = result {
                    assert_eq!(usuario.rol, Rol::Ambos);
                }
            }

            /// Verifica que no se pueda cambiar el rol de un usuario no registrado.
            #[ink::test]
            fn tests_cambiar_rol_usuario_no_registrado() {
                let mut marketplace = Marketplace::new();
                let caller = AccountId::from([0xAA; 32]);

                let result = marketplace.cambiar_rol(Rol::Ambos);
                assert_eq!(result, Err(ErrorSistema::UsuarioNoRegistrado));
            }
        }    

        mod tests_publicar {
            use super::*;

            /// Verifica que un vendedor pueda publicar un producto correctamente.
            #[ink::test]
            fn tests_publicar_correcto() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);
                let username = "agustin".to_string();
                let rol = Rol::Ambos;

                let _ = marketplace._registrar_usuario(caller.clone(), username, rol);

                let nombre = "Remera".to_string();
                let descripcion = "algodon".to_string();
                let precio = 12000;
                let categoria = Categoria::Ropa;
                let stock = 20;

                assert_eq!(
                    marketplace
                        ._publicar(
                            caller,
                            nombre,
                            descripcion,
                            precio,
                            categoria,
                            stock
                        )
                        .is_ok(),
                    true
                );
            }

            /// Verifica que un usuario no registrado no pueda publicar.
            #[ink::test]
            fn tests_publicar_usuario_no_encontrado() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);

                let nombre = "Remera".to_string();
                let descripcion = "algodon".to_string();
                let precio = 12000;
                let categoria = Categoria::Ropa;
                let stock = 20;

                let result = marketplace._publicar(
                    caller,
                    nombre,
                    descripcion,
                    precio,
                    categoria,
                    stock,
                );

                assert_eq!(result, Err(ErrorSistema::UsuarioNoRegistrado));
            }

            /// Verifica que un usuario sin rol de vendedor no pueda publicar.
            #[ink::test]
            fn tests_publicar_usuario_no_vendedor() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);
                let username = "agustin".to_string();
                let rol = Rol::Comprador;

                let _ = marketplace._registrar_usuario(caller.clone(), username, rol);

                let nombre = "Remera".to_string();
                let descripcion = "algodon".to_string();
                let precio = 12000;
                let categoria = Categoria::Ropa;
                let stock = 20;

                let result = marketplace._publicar(
                    caller,
                    nombre,
                    descripcion,
                    precio,
                    categoria,
                    stock,
                );

                assert_eq!(result, Err(ErrorSistema::UsuarioNoEsVendedor));
            }
        }

        mod tests_get_publicaciones_vendedor {
            use super::*;

            /// Verifica que un vendedor pueda obtener sus propias publicaciones.
            #[ink::test]
            fn tests_get_publicaciones_vendedor_correcto() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);
                let username = "agustin".to_string();
                let rol = Rol::Ambos;

                let _ = marketplace._registrar_usuario(caller.clone(), username, rol);

                let mut nombre = "Remera".to_string();
                let mut descripcion = "algodon".to_string();
                let mut precio = 12000;
                let mut categoria = Categoria::Ropa;
                let mut stock = 20;

                let _ = marketplace._publicar(
                    caller,
                    nombre,
                    descripcion,
                    precio,
                    categoria,
                    stock,
                );

                nombre = "Pantalon".to_string();
                descripcion = "Jean".to_string();
                precio = 20000;
                categoria = Categoria::Ropa;
                stock = 5;

                let _ = marketplace._publicar(
                    caller,
                    nombre,
                    descripcion,
                    precio,
                    categoria,
                    stock,
                );

                assert_eq!(
                    marketplace._get_publicaciones_vendedor(caller).is_ok(),
                    true
                );

                if let Ok(vec_publicaciones) = marketplace._get_publicaciones_vendedor(caller) {
                    assert_eq!(vec_publicaciones.len(), 2);
                }
            }

            /// Verifica que no se puedan obtener publicaciones de un vendedor no registrado.
            #[ink::test]
            fn tests_get_publicaciones_vendedor_usuario_no_encontrado() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);

                let result = marketplace._get_publicaciones_vendedor(caller);

                assert_eq!(result, Err(ErrorSistema::UsuarioNoRegistrado));
            }

            /// Verifica que un usuario sin rol de vendedor no pueda listar sus publicaciones.
            #[ink::test]
            fn tests_get_publicaciones_vendedor_usuario_no_vendedor() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);
                let username = "agustin".to_string();
                let rol = Rol::Comprador;

                let _ = marketplace._registrar_usuario(caller.clone(), username, rol);

                let result = marketplace._get_publicaciones_vendedor(caller);

                assert_eq!(result, Err(ErrorSistema::UsuarioNoEsVendedor));
            }
        }

        mod tests_get_publicaciones {
            use super::*;

            /// Verifica que se puedan obtener todas las publicaciones del sistema.
            #[ink::test]
            fn tests_get_publicaciones_correcto() {
                let mut marketplace = Marketplace::new();

                let caller1 = AccountId::from([0xAA; 32]);
                let username1 = "agustin".to_string();
                let rol1 = Rol::Ambos;

                let caller2 = AccountId::from([0xAA; 32]);
                let username2 = "agustin".to_string();
                let rol2 = Rol::Ambos;

                let _ = marketplace._registrar_usuario(caller1.clone(), username1, rol1);
                let _ = marketplace._registrar_usuario(caller2.clone(), username2, rol2);

                let mut nombre = "Remera".to_string();
                let mut descripcion = "algodon".to_string();
                let mut precio = 12000;
                let mut categoria = Categoria::Ropa;
                let mut stock = 20;

                let _ = marketplace._publicar(
                    caller1,
                    nombre,
                    descripcion,
                    precio,
                    categoria,
                    stock,
                );

                nombre = "Pantalon".to_string();
                descripcion = "Jean".to_string();
                precio = 20000;
                categoria = Categoria::Ropa;
                stock = 5;

                let _ = marketplace._publicar(
                    caller1,
                    nombre,
                    descripcion,
                    precio,
                    categoria,
                    stock,
                );

                nombre = "Notebook".to_string();
                descripcion = "Ryzen 7".to_string();
                precio = 200000;
                categoria = Categoria::Computacion;
                stock = 10;

                let _ = marketplace._publicar(
                    caller2,
                    nombre,
                    descripcion,
                    precio,
                    categoria,
                    stock,
                );

                assert_eq!(marketplace._get_publicaciones(caller1).is_ok(), true);

                if let Ok(vec_publicaciones) = marketplace._get_publicaciones(caller1) {
                    assert_eq!(vec_publicaciones.len(), 3);
                }
            }

            /// Verifica que un usuario no registrado no pueda obtener las publicaciones.
            #[ink::test]
            fn tests_get_publicaciones_usuario_no_encontrado() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);

                let result = marketplace._get_publicaciones(caller);

                assert_eq!(result, Err(ErrorSistema::UsuarioNoRegistrado));
            }
        }

        mod tests_ordenar_compra {
            use super::*;

            /// Verifica que un comprador pueda realizar una orden de compra correctamente.
            #[ink::test]
            fn tests_ordenar_compra_correcto_5_unidades() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);
                let username = "agustin".to_string();
                let rol = Rol::Ambos;

                let _ = marketplace._registrar_usuario(caller, username, rol);

                let nombre = "Remera".to_string();
                let descripcion = "algodon".to_string();
                let precio = 12000;
                let categoria = Categoria::Ropa;
                let stock = 20;

                let _ = marketplace._publicar(
                    caller,
                    nombre,
                    descripcion,
                    precio,
                    categoria,
                    stock,
                );

                let orden = marketplace._ordenar_compra(caller, 0_u32, 5_u32);
                assert!(orden.is_ok());
                assert!(marketplace.publicaciones[0].stock == 15);
            }

            /// Verifica que un usuario no registrado no pueda ordenar una compra.
            #[ink::test]
            fn tests_ordenar_compra_usuario_no_encontrado() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);

                let result = marketplace._ordenar_compra(caller, 0_u32, 5_u32);

                assert_eq!(result, Err(ErrorSistema::UsuarioNoRegistrado));
            }

            /// Verifica que un usuario sin rol de comprador no pueda ordenar una compra.
            #[ink::test]
            fn tests_ordenar_compra_usuario_no_comprador() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);
                let username = "agustin".to_string();
                let rol = Rol::Vendedor;

                let _ = marketplace._registrar_usuario(caller.clone(), username, rol);

                let result = marketplace._ordenar_compra(caller, 0_u32, 5_u32);

                assert_eq!(result, Err(ErrorSistema::UsuarioNoEsComprador));
            }

            /// Verifica que no se pueda ordenar una compra de una publicación inexistente.
            #[ink::test]
            fn tests_ordenar_compra_publicacion_no_existente() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);
                let username = "agustin".to_string();
                let rol = Rol::Ambos;

                let _ = marketplace._registrar_usuario(caller.clone(), username, rol);

                let mut nombre = "Remera".to_string();
                let mut descripcion = "algodon".to_string();
                let mut precio = 12000;
                let mut categoria = Categoria::Ropa;
                let mut stock = 20;

                let _ = marketplace._publicar(
                    caller,
                    nombre,
                    descripcion,
                    precio,
                    categoria,
                    stock,
                );

                let result = marketplace._ordenar_compra(caller, 1_u32, 5_u32);

                assert_eq!(result, Err(ErrorSistema::PublicacionNoExistente));
            }

            /// Verifica que no se pueda ordenar una compra si no hay stock suficiente.
            #[ink::test]
            fn tests_ordenar_compra_publicacion_sin_stock() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);
                let username = "agustin".to_string();
                let rol = Rol::Ambos;

                let _ = marketplace._registrar_usuario(caller, username, rol);

                let nombre = "Remera".to_string();
                let descripcion = "algodon".to_string();
                let precio = 12000;
                let categoria = Categoria::Ropa;
                let stock = 0;

                let _ = marketplace._publicar(
                    caller,
                    nombre,
                    descripcion,
                    precio,
                    categoria,
                    stock,
                );

                let result = marketplace._ordenar_compra(caller, 0_u32, 5_u32);

                assert_eq!(result, Err(ErrorSistema::PublicacionSinStock));
            }
        }

        mod tests_marcar_enviado {
            use super::*;

            /// Verifica que un vendedor pueda marcar una orden como enviada.
            #[ink::test]
            fn tests_marcar_enviado_correcto() {
                let mut marketplace = Marketplace::new();

                let vendedor = AccountId::from([0xAA; 32]);
                let comprador = AccountId::from([0xBB; 32]);

                let _ = marketplace._registrar_usuario(vendedor, "vendedor".to_string(), Rol::Vendedor);
                let _ = marketplace._registrar_usuario(comprador, "comprador".to_string(), Rol::Comprador);

                let _ = marketplace._publicar(
                    vendedor,
                    "Remera".to_string(),
                    "algodon".to_string(),
                    12000,
                    Categoria::Ropa,
                    20,
                );

                let _ = marketplace._ordenar_compra(comprador, 0_u32, 5_u32);

                let result = marketplace._marcar_enviado(vendedor, 0_u32);
                assert!(result.is_ok());
                assert_eq!(marketplace.ordenes_compra[0].estado, Estado::Enviada);
            }

            /// Verifica que un usuario no registrado no pueda marcar una orden como enviada.
            #[ink::test]
            fn tests_marcar_enviado_usuario_no_encontrado() {
                let mut marketplace = Marketplace::new();

                let vendedor = AccountId::from([0xAA; 32]);

                let result = marketplace._marcar_enviado(vendedor, 0_u32);

                assert_eq!(result, Err(ErrorSistema::UsuarioNoRegistrado));
            }

            /// Verifica que un usuario sin rol de vendedor no pueda marcar una orden como enviada.
            #[ink::test]
            fn tests_marcar_enviado_usuario_no_vendedor() {
                let mut marketplace = Marketplace::new();

                let comprador = AccountId::from([0xBB; 32]);

                let _ = marketplace._registrar_usuario(comprador, "comprador".to_string(), Rol::Comprador);

                let result = marketplace._marcar_enviado(comprador, 0_u32);

                assert_eq!(result, Err(ErrorSistema::UsuarioNoEsVendedor));
            }

            /// Verifica que no se pueda marcar como enviada una orden inexistente.
            #[ink::test]
            fn tests_marcar_enviado_orden_no_existe() {
                let mut marketplace = Marketplace::new();

                let vendedor = AccountId::from([0xAA; 32]);

                let _ = marketplace._registrar_usuario(vendedor, "vendedor".to_string(), Rol::Vendedor);

                let result = marketplace._marcar_enviado(vendedor, 0_u32);

                assert_eq!(result, Err(ErrorSistema::PublicacionNoExistente));
            }

            /// Verifica que solo el vendedor dueño de la publicación pueda marcar la orden como enviada.
            #[ink::test]
            fn tests_marcar_enviado_vendedor_no_es_dueno() {
                let mut marketplace = Marketplace::new();

                let vendedor1 = AccountId::from([0xAA; 32]);
                let vendedor2 = AccountId::from([0xCC; 32]);
                let comprador = AccountId::from([0xBB; 32]);

                let _ = marketplace._registrar_usuario(vendedor1, "vendedor1".to_string(), Rol::Vendedor);
                let _ = marketplace._registrar_usuario(vendedor2, "vendedor2".to_string(), Rol::Vendedor);
                let _ = marketplace._registrar_usuario(comprador, "comprador".to_string(), Rol::Comprador);

                let _ = marketplace._publicar(
                    vendedor1,
                    "Remera".to_string(),
                    "algodon".to_string(),
                    12000,
                    Categoria::Ropa,
                    20,
                );

                let _ = marketplace._ordenar_compra(comprador, 0_u32, 5_u32);

                let result = marketplace._marcar_enviado(vendedor2, 0_u32);

                assert_eq!(result, Err(ErrorSistema::NoEresVendedorDeLaOrden));
            }

            /// Verifica que no se pueda marcar como enviada una orden que ya fue enviada.
            #[ink::test]
            fn tests_marcar_enviado_orden_ya_enviada() {
                let mut marketplace = Marketplace::new();

                let vendedor = AccountId::from([0xAA; 32]);
                let comprador = AccountId::from([0xBB; 32]);

                let _ = marketplace._registrar_usuario(vendedor, "vendedor".to_string(), Rol::Vendedor);
                let _ = marketplace._registrar_usuario(comprador, "comprador".to_string(), Rol::Comprador);

                let _ = marketplace._publicar(
                    vendedor,
                    "Remera".to_string(),
                    "algodon".to_string(),
                    12000,
                    Categoria::Ropa,
                    20,
                );

                let _ = marketplace._ordenar_compra(comprador, 0_u32, 5_u32);

                let _ = marketplace._marcar_enviado(vendedor, 0_u32);

                let result = marketplace._marcar_enviado(vendedor, 0_u32);

                assert_eq!(result, Err(ErrorSistema::YaEnviada));
            }

            /// Verifica que un vendedor con rol `Ambos` pueda marcar una orden como enviada.
            #[ink::test]
            fn tests_marcar_enviado_vendedor_con_rol_ambos() {
                let mut marketplace = Marketplace::new();

                let vendedor = AccountId::from([0xAA; 32]);
                let comprador = AccountId::from([0xBB; 32]);

                let _ = marketplace._registrar_usuario(vendedor, "vendedor".to_string(), Rol::Ambos);
                let _ = marketplace._registrar_usuario(comprador, "comprador".to_string(), Rol::Comprador);

                let _ = marketplace._publicar(
                    vendedor,
                    "Remera".to_string(),
                    "algodon".to_string(),
                    12000,
                    Categoria::Ropa,
                    20,
                );

                let _ = marketplace._ordenar_compra(comprador, 0_u32, 5_u32);

                let result = marketplace._marcar_enviado(vendedor, 0_u32);

                assert!(result.is_ok());
                assert_eq!(marketplace.ordenes_compra[0].estado, Estado::Enviada);
            }

            /// Verifica que se puedan marcar múltiples órdenes como enviadas.
            #[ink::test]
            fn tests_marcar_enviado_multiples_ordenes() {
                let mut marketplace = Marketplace::new();

                let vendedor = AccountId::from([0xAA; 32]);
                let comprador = AccountId::from([0xBB; 32]);

                let _ = marketplace._registrar_usuario(vendedor, "vendedor".to_string(), Rol::Vendedor);
                let _ = marketplace._registrar_usuario(comprador, "comprador".to_string(), Rol::Comprador);

                let _ = marketplace._publicar(
                    vendedor,
                    "Remera".to_string(),
                    "algodon".to_string(),
                    12000,
                    Categoria::Ropa,
                    20,
                );

                let _ = marketplace._publicar(
                    vendedor,
                    "Pantalon".to_string(),
                    "jean".to_string(),
                    20000,
                    Categoria::Ropa,
                    10,
                );

                let _ = marketplace._ordenar_compra(comprador, 0_u32, 5_u32);
                let _ = marketplace._ordenar_compra(comprador, 1_u32, 3_u32);

                let result1 = marketplace._marcar_enviado(vendedor, 0_u32);
                let result2 = marketplace._marcar_enviado(vendedor, 1_u32);

                assert!(result1.is_ok());
                assert!(result2.is_ok());
                assert_eq!(marketplace.ordenes_compra[0].estado, Estado::Enviada);
                assert_eq!(marketplace.ordenes_compra[1].estado, Estado::Enviada);
            }
        }

        mod tests_marcar_recibido {
            use super::*;

            /// Verifica que un comprador pueda marcar una orden como recibida.
            #[ink::test]
            fn tests_marcar_recibido_correcto() {
                let mut marketplace = Marketplace::new();

                let vendedor = AccountId::from([0xAA; 32]);
                let comprador = AccountId::from([0xBB; 32]);

                let _ = marketplace._registrar_usuario(vendedor, "vendedor".to_string(), Rol::Vendedor);
                let _ = marketplace._registrar_usuario(comprador, "comprador".to_string(), Rol::Comprador);

                let _ = marketplace._publicar(
                    vendedor,
                    "Remera".to_string(),
                    "algodon".to_string(),
                    12000,
                    Categoria::Ropa,
                    20,
                );

                let _ = marketplace._ordenar_compra(comprador, 0_u32, 5_u32);
                // vendedor marca enviado
                let _ = marketplace._marcar_enviado(vendedor, 0_u32);

                let result = marketplace._marcar_recibido(comprador, 0_u32);
                assert!(result.is_ok());
                assert_eq!(marketplace.ordenes_compra[0].estado, Estado::Recibida);
            }

            /// Verifica que un usuario no registrado no pueda marcar una orden como recibida.
            #[ink::test]
            fn tests_marcar_recibido_usuario_no_encontrado() {
                let mut marketplace = Marketplace::new();

                let comprador = AccountId::from([0xBB; 32]);

                let result = marketplace._marcar_recibido(comprador, 0_u32);

                assert_eq!(result, Err(ErrorSistema::UsuarioNoRegistrado));
            }

            /// Verifica que un usuario sin rol de comprador no pueda marcar una orden como recibida.
            #[ink::test]
            fn tests_marcar_recibido_usuario_no_comprador() {
                let mut marketplace = Marketplace::new();

                let vendedor = AccountId::from([0xAA; 32]);

                let _ = marketplace._registrar_usuario(vendedor, "vendedor".to_string(), Rol::Vendedor);

                let result = marketplace._marcar_recibido(vendedor, 0_u32);

                assert_eq!(result, Err(ErrorSistema::UsuarioNoEsComprador));
            }

            /// Verifica que no se pueda marcar como recibida una orden inexistente.
            #[ink::test]
            fn tests_marcar_recibido_orden_no_existe() {
                let mut marketplace = Marketplace::new();

                let comprador = AccountId::from([0xBB; 32]);
                let _ = marketplace._registrar_usuario(comprador, "comprador".to_string(), Rol::Comprador);

                let result = marketplace._marcar_recibido(comprador, 0_u32);

                assert_eq!(result, Err(ErrorSistema::PublicacionNoExistente));
            }

            /// Verifica que solo el comprador dueño de la orden pueda marcarla como recibida.
            #[ink::test]
            fn tests_marcar_recibido_no_es_dueno() {
                let mut marketplace = Marketplace::new();

                let vendedor = AccountId::from([0xAA; 32]);
                let comprador1 = AccountId::from([0xBB; 32]);
                let comprador2 = AccountId::from([0xCC; 32]);

                let _ = marketplace._registrar_usuario(vendedor, "vendedor".to_string(), Rol::Vendedor);
                let _ = marketplace._registrar_usuario(comprador1, "comprador1".to_string(), Rol::Comprador);
                let _ = marketplace._registrar_usuario(comprador2, "comprador2".to_string(), Rol::Comprador);

                let _ = marketplace._publicar(
                    vendedor,
                    "Remera".to_string(),
                    "algodon".to_string(),
                    12000,
                    Categoria::Ropa,
                    20,
                );

                let _ = marketplace._ordenar_compra(comprador1, 0_u32, 2_u32);
                let _ = marketplace._marcar_enviado(vendedor, 0_u32);

                let result = marketplace._marcar_recibido(comprador2, 0_u32);

                assert_eq!(result, Err(ErrorSistema::NoEresCompradorDeLaOrden));
            }

            /// Verifica que no se pueda marcar como recibida una orden que aún está pendiente (no enviada).
            #[ink::test]
            fn tests_marcar_recibido_orden_pendiente() {
                let mut marketplace = Marketplace::new();

                let vendedor = AccountId::from([0xAA; 32]);
                let comprador = AccountId::from([0xBB; 32]);

                let _ = marketplace._registrar_usuario(vendedor, "vendedor".to_string(), Rol::Vendedor);
                let _ = marketplace._registrar_usuario(comprador, "comprador".to_string(), Rol::Comprador);

                let _ = marketplace._publicar(
                    vendedor,
                    "Remera".to_string(),
                    "algodon".to_string(),
                    12000,
                    Categoria::Ropa,
                    20,
                );

                let _ = marketplace._ordenar_compra(comprador, 0_u32, 1_u32);
                // vendedor no marca enviado
                let result = marketplace._marcar_recibido(comprador, 0_u32);

                assert_eq!(result, Err(ErrorSistema::OrdenPendiente));
            }

            /// Verifica que no se pueda marcar como recibida una orden que ya fue recibida.
            #[ink::test]
            fn tests_marcar_recibido_ya_recibida() {
                let mut marketplace = Marketplace::new();

                let vendedor = AccountId::from([0xAA; 32]);
                let comprador = AccountId::from([0xBB; 32]);

                let _ = marketplace._registrar_usuario(vendedor, "vendedor".to_string(), Rol::Vendedor);
                let _ = marketplace._registrar_usuario(comprador, "comprador".to_string(), Rol::Comprador);

                let _ = marketplace._publicar(
                    vendedor,
                    "Remera".to_string(),
                    "algodon".to_string(),
                    12000,
                    Categoria::Ropa,
                    20,
                );

                let _ = marketplace._ordenar_compra(comprador, 0_u32, 1_u32);
                let _ = marketplace._marcar_enviado(vendedor, 0_u32);
                let _ = marketplace._marcar_recibido(comprador, 0_u32);

                let result = marketplace._marcar_recibido(comprador, 0_u32);

                assert_eq!(result, Err(ErrorSistema::YaRecibido));
            }

            /// Verifica que no se pueda marcar como recibida una orden cancelada.
            #[ink::test]
            fn tests_marcar_recibido_orden_cancelada() {
                let mut marketplace = Marketplace::new();

                let vendedor = AccountId::from([0xAA; 32]);
                let comprador = AccountId::from([0xBB; 32]);

                let _ = marketplace._registrar_usuario(vendedor, "vendedor".to_string(), Rol::Vendedor);
                let _ = marketplace._registrar_usuario(comprador, "comprador".to_string(), Rol::Comprador);

                let _ = marketplace._publicar(
                    vendedor,
                    "Remera".to_string(),
                    "algodon".to_string(),
                    12000,
                    Categoria::Ropa,
                    20,
                );

                let _ = marketplace._ordenar_compra(comprador, 0_u32, 1_u32);
                // Simular que la orden fue cancelada
                marketplace.ordenes_compra[0].estado = Estado::Cancelada;

                let result = marketplace._marcar_recibido(comprador, 0_u32);

                assert_eq!(result, Err(ErrorSistema::OrdenCancelada));
            }
        }

        mod tests_get_ordenes_comprador {
            use super::*;

            /// Verifica que un comprador pueda obtener sus órdenes de compra.
            #[ink::test]
            fn tests_get_ordenes_comprador_correcto() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);
                let username = "agustin".to_string();
                let rol = Rol::Ambos;

                let _ = marketplace._registrar_usuario(caller, username, rol);

                let mut nombre = "Remera".to_string();
                let mut descripcion = "algodon".to_string();
                let mut precio = 12000;
                let mut categoria = Categoria::Ropa;
                let mut stock = 20;

                let _ = marketplace._publicar(
                    caller,
                    nombre,
                    descripcion,
                    precio,
                    categoria,
                    stock,
                );

                let _ = marketplace._ordenar_compra(caller, 0_u32, 5_u32);

                nombre = "Pantalon".to_string();
                descripcion = "Jean".to_string();
                precio = 20000;
                categoria = Categoria::Ropa;
                stock = 5;

                let _ = marketplace._publicar(
                    caller,
                    nombre,
                    descripcion,
                    precio,
                    categoria,
                    stock,
                );

                let _ = marketplace._ordenar_compra(caller, 1_u32, 2_u32);

                assert!(marketplace._get_ordenes_comprador(caller).is_ok());

                if let Ok(vec_ordenes) = marketplace._get_ordenes_comprador(caller) {
                    assert_eq!(vec_ordenes.len(), 2);
                }
            }

            /// Verifica que un usuario no registrado no pueda obtener órdenes de compra.
            #[ink::test]
            fn tests_get_ordenes_comprador_usuario_no_encontrado() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);

                let result = marketplace._get_ordenes_comprador(caller);

                assert_eq!(result, Err(ErrorSistema::UsuarioNoRegistrado));
            }

            /// Verifica que un usuario sin rol de comprador no pueda obtener órdenes de compra.
            #[ink::test]
            fn tests_get_ordenes_comprador_usuario_no_comprador() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);
                let username = "agustin".to_string();
                let rol = Rol::Vendedor;

                let _ = marketplace._registrar_usuario(caller.clone(), username, rol);

                let result = marketplace._get_ordenes_comprador(caller);

                assert_eq!(result, Err(ErrorSistema::UsuarioNoEsComprador));
            }
        }

        mod tests_get_ordenes {
            use super::*;

            /// Verifica que se puedan obtener todas las órdenes de compra del sistema.
            #[ink::test]
            fn tests_get_ordenes_correcto() {
                let mut marketplace = Marketplace::new();

                let caller1 = AccountId::from([0xAA; 32]);
                let username1 = "agustin".to_string();
                let rol1 = Rol::Ambos;

                let caller2 = AccountId::from([0xBB; 32]);
                let username2 = "juan".to_string();
                let rol2 = Rol::Ambos;

                let _ = marketplace._registrar_usuario(caller1, username1, rol1);
                let _ = marketplace._registrar_usuario(caller2, username2, rol2);

                let mut nombre = "Remera".to_string();
                let mut descripcion = "algodon".to_string();
                let mut precio = 12000;
                let mut categoria = Categoria::Ropa;
                let mut stock = 20;

                let _ = marketplace._publicar(
                    caller1,
                    nombre,
                    descripcion,
                    precio,
                    categoria,
                    stock,
                );

                let _ = marketplace._ordenar_compra(caller2, 0_u32, 5_u32);

                nombre = "Pantalon".to_string();
                descripcion = "Jean".to_string();
                precio = 20000;
                categoria = Categoria::Ropa;
                stock = 5;

                let _ = marketplace._publicar(
                    caller1,
                    nombre,
                    descripcion,
                    precio,
                    categoria,
                    stock,
                );

                let _ = marketplace._ordenar_compra(caller2, 1_u32, 2_u32);

                assert!(marketplace._get_ordenes(caller1).is_ok());

                if let Ok(vec_ordenes) = marketplace._get_ordenes(caller1) {
                    assert_eq!(vec_ordenes.len(), 2);
                }
            }

            /// Verifica que un usuario no registrado no pueda obtener todas las órdenes.
            #[ink::test]
            fn tests_get_ordenes_usuario_no_encontrado() {
                let marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);

                let result = marketplace._get_ordenes(caller);

                assert_eq!(result, Err(ErrorSistema::UsuarioNoRegistrado));
            }

            /// Verifica que se retorne una lista vacía si no hay órdenes en el sistema.
            #[ink::test]
            fn tests_get_ordenes_sin_ordenes() {
                let mut marketplace = Marketplace::new();

                let caller = AccountId::from([0xAA; 32]);
                let username = "agustin".to_string();
                let rol = Rol::Ambos;

                let _ = marketplace._registrar_usuario(caller, username, rol);

                let result = marketplace._get_ordenes(caller);

                assert!(result.is_ok());
                if let Ok(vec_ordenes) = result {
                    assert_eq!(vec_ordenes.len(), 0);
                }
            }
        }

        mod tests_cancelar_orden {
            use super::*;

            /// Verifica que un comprador pueda solicitar la cancelación de una orden.
            #[ink::test]
            fn tests_cancelar_orden_solicitud_comprador() {
                let mut marketplace = Marketplace::new();
                let vendedor = AccountId::from([0xAA; 32]);
                let comprador = AccountId::from([0xBB; 32]);

                // Setup usuario Vendedor
                let _ = marketplace._registrar_usuario(vendedor, "vendedor".to_string(), Rol::Vendedor);
                // Crea una publicacion
                let _ = marketplace._publicar(vendedor, "Item".to_string(), "Desc".to_string(), 100, Categoria::Computacion, 10);

                // Setup usuario Comprador
                let _ = marketplace._registrar_usuario(comprador, "comprador".to_string(), Rol::Comprador);
                
                // Ordenar compra
                let _ = marketplace._ordenar_compra(comprador, 0, 2);

                // Solicitar cancelacion 
                let result = marketplace._cancelar_orden(comprador, 0);
                assert!(result.is_ok());
                
                if let Ok(orden) = result {
                    assert_eq!(orden.peticion_cancelacion, true);
                    assert_eq!(orden.estado, Estado::Pendiente);
                }
            }

            /// Verifica que un vendedor pueda aprobar la cancelación solicitada por el comprador y que el stock se restaure.
            #[ink::test]
            fn tests_cancelar_orden_aprobacion_vendedor() {
                let mut marketplace = Marketplace::new();
                let vendedor = AccountId::from([0xAA; 32]);
                let comprador = AccountId::from([0xBB; 32]);

                // Setup usuario Vendedor
                let _ = marketplace._registrar_usuario(vendedor, "vendedor".to_string(), Rol::Vendedor);
                // Crea una publicacion
                let _ = marketplace._publicar(vendedor, "Item".to_string(), "Desc".to_string(), 100, Categoria::Computacion, 10);

                // Setup usuario Comprador
                let _ = marketplace._registrar_usuario(comprador, "comprador".to_string(), Rol::Comprador);
                
                // Ordenar compra 
                let _ = marketplace._ordenar_compra(comprador, 0, 2);

                // Solicitar cancelacion 
                let _ = marketplace._cancelar_orden(comprador, 0);

                // Aprobar cancelacion (caller Vendedor)
                let result = marketplace._cancelar_orden(vendedor, 0);
                assert!(result.is_ok());

                if let Ok(orden) = result {
                    assert_eq!(orden.estado, Estado::Cancelada);
                }

                // Verificar que el stock se restablezca
                let result_pub = marketplace._get_publicaciones_vendedor(vendedor);
                assert!(result_pub.is_ok());
                
                if let Ok(publicaciones) = result_pub {
                    assert_eq!(publicaciones[0].stock, 10); // 10 - 2 + 2 = 10
                }
            }

            /// Verifica que el vendedor no pueda cancelar una orden si el comprador no lo ha solicitado.
            #[ink::test]
            fn tests_cancelar_orden_vendedor_no_solicitud() {
                let mut marketplace = Marketplace::new();
                let vendedor = AccountId::from([0xAA; 32]);
                let comprador = AccountId::from([0xBB; 32]);

                // Setup usuario Vendedor
                let _ = marketplace._registrar_usuario(vendedor, "vendedor".to_string(), Rol::Vendedor);
                let _ = marketplace._publicar(vendedor, "Item".to_string(), "Desc".to_string(), 100, Categoria::Computacion, 10);

                // Setup usuario Comprador
                let _ = marketplace._registrar_usuario(comprador, "comprador".to_string(), Rol::Comprador);
                
                // Ordenar compra
                let _ = marketplace._ordenar_compra(comprador, 0, 2);

                // Intentar aprobar cancelacion sin solicitud del comprador
                let result = marketplace._cancelar_orden(vendedor, 0);
                assert_eq!(result, Err(ErrorSistema::PeticionNoSolicitada));
            }

            /// Verifica que no se pueda cancelar una orden inexistente.
            #[ink::test]
            fn tests_cancelar_orden_orden_no_existente() {
                let mut marketplace = Marketplace::new();
                let vendedor = AccountId::from([0xAA; 32]);
                
                // Setup usuario Vendedor
                let _ = marketplace._registrar_usuario(vendedor, "vendedor".to_string(), Rol::Vendedor);
                
                // Intentar cancelar orden inexistente
                let result = marketplace._cancelar_orden(vendedor, 999);
                assert_eq!(result, Err(ErrorSistema::PublicacionNoExistente));
            }

            /// Verifica que no se pueda cancelar una orden que no está en estado `Pendiente`.
            #[ink::test]
            fn tests_cancelar_orden_estado_no_pendiente() {
                let mut marketplace = Marketplace::new();
                let vendedor = AccountId::from([0xAA; 32]);
                let comprador = AccountId::from([0xBB; 32]);

                // Setup usuario Vendedor
                let _ = marketplace._registrar_usuario(vendedor, "vendedor".to_string(), Rol::Vendedor);
                let _ = marketplace._publicar(vendedor, "Item".to_string(), "Desc".to_string(), 100, Categoria::Computacion, 10);

                // Setup usuario Comprador
                let _ = marketplace._registrar_usuario(comprador, "comprador".to_string(), Rol::Comprador);
                
                // Ordenar compra
                let _ = marketplace._ordenar_compra(comprador, 0, 2);

                // Solicitar cancelacion
                let _ = marketplace._cancelar_orden(comprador, 0);

                // Aprobar cancelacion
                let _ = marketplace._cancelar_orden(vendedor, 0);

                // Intentar cancelar de nuevo (ya está cancelada)
                let result = marketplace._cancelar_orden(comprador, 0);
                assert_eq!(result, Err(ErrorSistema::OrdenNoPendiente));
            }

            /// Verifica que un usuario sin permisos (ni comprador ni vendedor) no pueda cancelar la orden.
            #[ink::test]
            fn tests_cancelar_orden_sin_permisos() {
                let mut marketplace = Marketplace::new();
                let vendedor = AccountId::from([0xAA; 32]);
                let comprador = AccountId::from([0xBB; 32]);
                let otro_usuario = AccountId::from([0xCC; 32]);

                // Setup usuario Vendedor
                let _ = marketplace._registrar_usuario(vendedor, "vendedor".to_string(), Rol::Vendedor);
                let _ = marketplace._publicar(vendedor, "Item".to_string(), "Desc".to_string(), 100, Categoria::Computacion, 10);

                // Setup usuario Comprador
                let _ = marketplace._registrar_usuario(comprador, "comprador".to_string(), Rol::Comprador);
                
                // Ordenar compra
                let _ = marketplace._ordenar_compra(comprador, 0, 2);

                // Setup otro usuario
                let _ = marketplace._registrar_usuario(otro_usuario, "otro".to_string(), Rol::Comprador);

                // Intentar cancelar orden ajena
                let result = marketplace._cancelar_orden(otro_usuario, 0);
                assert_eq!(result, Err(ErrorSistema::SinPermisos));
            }

            #[ink::test]
            fn tests_cancelar_orden_usuario_no_registrado() {
                let mut marketplace = Marketplace::new();
                let no_registrado = AccountId::from([0xDD; 32]);

                // Intentar cancelar sin estar registrado
                let result = marketplace._cancelar_orden(no_registrado, 0);
                assert_eq!(result, Err(ErrorSistema::UsuarioNoRegistrado));
            }
        }
    }
}
